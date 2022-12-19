use crate::AppState;

#[derive(Default, derive_sql::DeriveSql, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Message {
  #[serde(default = "crate::gen_key")]
  id: String,
  originator_id: String,
  message: String,
}

#[derive(serde::Deserialize)]
pub struct Info {
  originator_id: Option<String>
}

impl Into<Select> for Info {
  fn into(self) -> Select {
    match self.originator_id {
      Some(v) => Filter::OriginatorIdEqual(v).into(),
      None => SelectBuilder::default().build(),
    }
  }
}

#[cfg(feature = "lambda")]
impl From<query_map::QueryMap> for Info {
  fn from(query: query_map::QueryMap) -> Info {
    Info { originator_id: query.first("originator_id").map(str::to_string) }
  }
}

impl Message {
  crate::macros::db::post!(Message, MessageSql, AppState);
  crate::macros::db::get!(Message, MessageSql, AppState);
  crate::macros::db::update!(Message, MessageSql, AppState);
  crate::macros::db::delete!(Message, MessageSql, AppState);
  crate::macros::db::list!(Message, MessageSql, AppState, Info);
}

#[cfg(feature = "lambda")]
pub mod apigw {
  use super::*;
  use crate::lambda::Request;
  crate::macros::lambda::post!(Message, MessageSql, AppState);
  crate::macros::lambda::list!(Message, MessageSql, AppState, Info);

  pub async fn handler(request: Request, app_state: AppState) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
    match request {
      Request::List { query } => list(app_state, query.into()).await,
      Request::Post { body  } => post(app_state, body).await,
      _ => Err(format!("Request is not supported").into()),
    }
  }
}

#[cfg(feature = "actix")]
pub mod actix {
  use super::*;
  crate::macros::actix::post!(Message, MessageSql, AppState);
  crate::macros::actix::list!(Message, MessageSql, AppState, Info);

  pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
       actix_web::web::resource("")
       .route(actix_web::web::get().to(list))
       .route(actix_web::web::post().to(post))
     );
  }
}
