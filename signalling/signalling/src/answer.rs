use crate::AppState;

#[derive(Default, derive_sql::DeriveSql, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Answer {
  #[serde(default = "crate::gen_key")]
  id: String,
  offer_id: String,
  name: String,
  #[serde(default = "polite")]
  polite: bool,
  #[serde(default = "crate::make_old_date")]
  timestamp: chrono::DateTime<chrono::Utc>,
}

fn polite() -> bool { false }

#[derive(serde::Deserialize)]
pub struct Info {
  offer_id: Option<String>,
}
impl Into<Option<Filter>> for Info {
  fn into(self) -> Option<Filter> {
    self.offer_id
        .map(|v| Filter::OfferIdEqual(v))
  }
}
impl Into<Select> for Info {
  fn into(self) -> Select {
    match self.offer_id {
      Some(v) => Filter::OfferIdEqual(v).into(),
      None => Filter::OfferIdEqual("123456789".to_string()).into(), // Equivalent to a Filter::None (to be implemented in derive_sql)
    }
  }
}

#[cfg(feature = "lambda")]
impl From<query_map::QueryMap> for Info {
  fn from(query: query_map::QueryMap) -> Info {
    Info { offer_id: query.first("offer_id").map(str::to_string) }
  }
}

impl Answer {
  crate::macros::db::post!(Answer, AnswerSql, AppState);
  crate::macros::db::get!(Answer, AnswerSql, AppState);
  crate::macros::db::update!(Answer, AnswerSql, AppState);
  crate::macros::db::delete!(Answer, AnswerSql, AppState);
  crate::macros::db::list!(Answer, AnswerSql, AppState, Info);
  crate::macros::db::count!(Answer, AnswerSql, AppState);
}

#[cfg(feature = "lambda")]
pub mod apigw {
  use super::*;
  use crate::lambda::Request;
  crate::macros::lambda::post!(Answer, AnswerSql, AppState);
  crate::macros::lambda::get!(Answer, AnswerSql, AppState);
  crate::macros::lambda::update!(Answer, AnswerSql, AppState);
  crate::macros::lambda::delete!(Answer, AnswerSql, AppState);
  crate::macros::lambda::list!(Answer, AnswerSql, AppState, Info);
  crate::macros::lambda::count!(Answer, AnswerSql, AppState);

  pub async fn handler(request: Request, app_state: AppState) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
    match request {
      Request::List { query } => list(app_state, Info::from(query)).await,
      Request::Post { body } => post(app_state, body).await,
      Request::Get { key } => get(app_state, key).await,
      // Request::Update { key, body } => update(app_state, key, body).await,
      // Request::Delete { key } => delete(app_state, key).await,
      _ => Err(format!("Request is not supported").into()),
    }
  }
}

#[cfg(feature = "actix")]
pub mod actix {
  use super::*;
  crate::macros::actix::post!(Answer, AnswerSql, AppState);
  crate::macros::actix::get!(Answer, AnswerSql, AppState);
  crate::macros::actix::update!(Answer, AnswerSql, AppState);
  crate::macros::actix::delete!(Answer, AnswerSql, AppState);
  crate::macros::actix::list!(Answer, AnswerSql, AppState, Info);
  crate::macros::actix::count!(Answer, AnswerSql, AppState);

  pub fn config_count(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
       actix_web::web::resource("")
       .route(actix_web::web::get().to(count))
     );
  }

  pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
       actix_web::web::resource("")
       .route(actix_web::web::get().to(list))
       .route(actix_web::web::post().to(post))
     )
     .service(
       actix_web::web::resource("/{key}")
       .route(actix_web::web::get().to(get))
       // .route(actix_web::web::patch().to(update))
       // .route(actix_web::web::delete().to(delete))
     );
  }
}
