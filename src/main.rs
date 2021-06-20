use serde::Deserialize;
use yew::{
  format::{Json, Nothing},
  prelude::*,
  services::fetch::{FetchService, FetchTask, Request, Response}
};

#[derive(Deserialize, Debug, Clone)]
pub struct PingResponse {
  AndroidLatestVersion: String,
  AndroidMinVersion: String,
  DesktopLatestVersion: String,
  DesktopMinVersion: String,
  IosLatestVersion: String,
  IosMinVersion: String,
  status: String
}

#[derive(Debug)]
pub enum Msg {
  Ping,
  ReceiveResponse(Result<PingResponse, anyhow::Error>)
}

#[derive(Debug)]
pub struct SignUpEmail {
  link: ComponentLink<Self>,
  ping_response: Option<PingResponse>,
  fetch_task: Option<FetchTask>,
  error: Option<String>
}

impl SignUpEmail {
  fn view_ping_response(&self) -> Html {
    match self.ping_response {
      None => {
        html! {
          <div>{"No data yet!"}</div>
        }
      },
      Some(ref ping_response) => {
        html! {
          <div>{format!("status: {}, latest version: {}", ping_response.status, ping_response.DesktopLatestVersion)}</div>
        }
      }
    }
  }
}

impl Component for SignUpEmail {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      fetch_task: None,
      ping_response: None,
      error: None
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Ping => {
        let request = Request::get("http://localhost:8065/api/v4/system/ping")
          .body(Nothing)
          .expect("Could not build request");

        let callback = self.link.callback(|response: Response<Json<Result<PingResponse, anyhow::Error>>>| {
          let Json(data) = response.into_body();
          Msg::ReceiveResponse(data)
        });

        let task = FetchService::fetch(request, callback).expect("Failed to start request!");

        self.fetch_task = Some(task);
        true
      },
      Msg::ReceiveResponse(response) => {
        match response {
          Ok(resp) => {
            self.ping_response = Some(resp);
          },
          Err(error) => {
            self.error = Some(error.to_string());
          }
        }
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false 
  }

  fn view(&self) -> Html {
    html! {
      <div>
        { self.view_ping_response() }
        <button onclick=self.link.callback(|_| Msg::Ping)>{"Signup"}</button>
      </div>
    }
  }
}

fn main() {
  yew::start_app::<SignUpEmail>();
}
