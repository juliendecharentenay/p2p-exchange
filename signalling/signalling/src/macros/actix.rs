
macro_rules! list {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// List all
    pub async fn list(data: actix_web::web::Data<$data>) -> impl actix_web::Responder {
      match $class::list(&data).await {
        Ok(items) => actix_web::HttpResponse::Ok().json(items),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
      }
    }
  };

  ( $class:ident, $class_sql:ident, $data:ident, $info:ident ) => {
    /// List with a query string
    pub async fn list(data: actix_web::web::Data<$data>, info: actix_web::web::Query<$info>) -> impl actix_web::Responder {
      match $class::list(&data, info.into_inner()).await {
        Ok(items) => actix_web::HttpResponse::Ok().json(items),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
      }
    }
  };
}
pub(crate) use list;

macro_rules! post {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn post(data: actix_web::web::Data<$data>, body: String) -> impl actix_web::Responder {
      match $class::post(&data, &body).await {
        Ok(item) => actix_web::HttpResponse::Created().json(item),
        Err(e) => {
          log::error!("{:?}", e);
          actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e))
        },
      }
    }
  };
}
pub(crate) use post;

macro_rules! get {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn get(data: actix_web::web::Data<$data>, path: actix_web::web::Path<String>) -> impl actix_web::Responder {
      let id = path.into_inner();
      match $class::get(&data, &id).await {
        Ok(Some(item)) => actix_web::HttpResponse::Ok().json(item),
        Ok(None) => actix_web::HttpResponse::NotFound().body(format!("Item {} not found", id)),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
      }
    }
  };
}
pub(crate) use get;

macro_rules! update {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn update(data: actix_web::web::Data<$data>, path: actix_web::web::Path<String>, body: String) -> impl actix_web::Responder {
      let id = path.into_inner();
      match $class::update(&data, &id, &body).await {
        Ok(Some(item)) => actix_web::HttpResponse::Ok().json(item),
        Ok(None) => actix_web::HttpResponse::NotFound().body(format!("Item {} not found", id)),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
      }
    }
  };
}
pub(crate) use update;

macro_rules! delete {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn delete(data: actix_web::web::Data<$data>, path: actix_web::web::Path<String>) -> impl actix_web::Responder {
      let id = path.into_inner();
      match $class::delete(&data, &id).await {
        Ok(Some(item)) => actix_web::HttpResponse::Ok().json(item),
        Ok(None) => actix_web::HttpResponse::NotFound().body(format!("Item {} not found", id)),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(format!("{:#?}", e)),
      }
    }
  };
}
pub(crate) use delete;

