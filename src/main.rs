use serde::Deserialize;
use serde_json::json;
use yew::{
  format::{Json, Nothing},
  prelude::*,
  services::fetch::{FetchService, FetchTask, Request, Response}
};

#[derive(Deserialize, Debug, Clone)]
pub struct PingResponse {
  status: String
}

#[derive(Debug)]
pub enum Msg {
  RecvUserEmail(Result<ChangeData, anyhow::Error>),
  SignUp,
  ReceiveResponse(Result<PingResponse, anyhow::Error>)
}

#[derive(Debug)]
pub struct User {
  email: Option<String>,
  username: Option<String>,
  password: Option<String>
}

#[derive(Debug)]
pub struct SignUpEmail {
  link: ComponentLink<Self>,
  ping_response: Option<PingResponse>,
  fetch_task: Option<FetchTask>,
  error: Option<String>,
  user: User
}

impl SignUpEmail {
  fn view_ping_response(&self) -> Html {
    let email = match &self.user.email { Some(em) => String::from(em), None => String::from("No email yet") };
    html! { <div>{format!("Your email is: {}", email)}</div> }
  }

  fn handle_messages(&mut self, msg: Msg) -> bool {
    match msg {
      Msg::SignUp => {
        let payload = json!({
          "user": {
            "email": self.user.email,
            "password": self.user.email,
            "username": self.user.email
          }
        });
        let request = Request::post("http://localhost:8065/api/v4/users")
          .body(Json(&payload))
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
      },
      Msg::RecvUserEmail(maybe_event) => {
        match maybe_event {
          Ok(event) => {
            match event {
              ChangeData::Value(email) => {
                self.user.email = Some(email);
              },
              _ => {}
            };
            true
          },
          Err(error) => {
            self.error = Some(error.to_string());
            false
          }
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
      error: None,
      user: User {
        email: None,
        password: None,
        username: None
      }
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    self.handle_messages(msg)
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false 
  }

  fn view(&self) -> Html {
    html! {
      <div>
        { self.view_ping_response() }
        <input onchange=self.link.callback(|e: ChangeData| Msg::RecvUserEmail(Ok(e)))/>
        <button onclick=self.link.callback(|_| Msg::SignUp)>{"Signup"}</button>
      </div>
    }
  }
}

fn main() {
  yew::start_app::<SignUpEmail>();
}
