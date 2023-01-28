use crate::AppState;

#[derive(Default, derive_sql::DeriveSql, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Offer {
  #[serde(default = "crate::gen_key")]
  id: String,
  name: String,
  #[serde(default = "polite")]
  polite: bool,
  #[serde(default = "crate::make_old_date")]
  timestamp: chrono::DateTime<chrono::Utc>,
}

fn polite() -> bool { true }

impl Offer {
  crate::macros::db::post!(Offer, OfferSql, AppState);
  crate::macros::db::get!(Offer, OfferSql, AppState);
  crate::macros::db::update!(Offer, OfferSql, AppState);
  crate::macros::db::delete!(Offer, OfferSql, AppState);
  crate::macros::db::list!(Offer, OfferSql, AppState);
  crate::macros::db::count!(Offer, OfferSql, AppState);
}

#[cfg(feature = "lambda")]
pub mod apigw {
  use super::*;
  use crate::lambda::Request;
  crate::macros::lambda::list!(Offer, OfferSql, AppState);
  crate::macros::lambda::post!(Offer, OfferSql, AppState);
  crate::macros::lambda::get!(Offer, OfferSql, AppState);
  crate::macros::lambda::update!(Offer, OfferSql, AppState);
  crate::macros::lambda::delete!(Offer, OfferSql, AppState);
  crate::macros::lambda::count!(Offer, OfferSql, AppState);

  pub async fn handler(request: Request, app_state: AppState) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
    match request {
      // Request::List { ..    } => list(app_state).await,
      Request::Post { body  } => post(app_state, body).await,
      Request::Get  { key   } => get(app_state, key).await,
      // Request::Update { key, body } => update(app_state, key, body).await,
      // Request::Delete { key } => delete(app_state, key).await,
      Request::Count => count(app_state).await,
      _ => Err(format!("Request is not supported").into()),
    }
  }
}

#[cfg(feature = "actix")]
pub mod actix {
  use super::*;
  crate::macros::actix::list!(Offer, OfferSql, AppState);
  crate::macros::actix::post!(Offer, OfferSql, AppState);
  crate::macros::actix::get!(Offer, OfferSql, AppState);
  crate::macros::actix::update!(Offer, OfferSql, AppState);
  crate::macros::actix::delete!(Offer, OfferSql, AppState);
  crate::macros::actix::count!(Offer, OfferSql, AppState);

  pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
       actix_web::web::resource("")
       // .route(actix_web::web::get().to(list))
       .route(actix_web::web::post().to(post))
     )
     .service(
       actix_web::web::resource("/count")
       .route(actix_web::web::get().to(count))
     )
     .service(
       actix_web::web::resource("/{key}")
       .route(actix_web::web::get().to(get))
       // .route(actix_web::web::patch().to(update))
       // .route(actix_web::web::delete().to(delete))
     );
  }
}
