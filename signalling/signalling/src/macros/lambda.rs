macro_rules! count {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// List all
    pub async fn count(data: $data) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      let count = $class::count(&data).await?;
      let body = serde_json::to_string(&count)?;
      let r = lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(body)?;
      Ok(r)
    }
  };
}
pub(crate) use count;


macro_rules! list {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    /// List all
    pub async fn list(data: $data) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      let items = $class::list(&data).await?;
      let body = serde_json::to_string(&items)?;
      let r = lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(body)?;
      Ok(r)
    }
  };

  ( $class:ident, $class_sql:ident, $data:ident, $info:ident ) => {
    /// List with a query string
    pub async fn list(data: $data, info: $info) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      let items = $class::list(&data, info).await?;
      let body = serde_json::to_string(&items)?;
      let r = lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(body)?;
      Ok(r)
    }
  };
}
pub(crate) use list;

macro_rules! post {
  ( $class:ident, $calss_sql:ident, $data:ident ) => {
    pub async fn post(data: $data, body: String) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      let item = $class::post(&data, &body).await?;
      let body = serde_json::to_string(&item)?;
      Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(body)?)
    }
  };
}
pub(crate) use post;

macro_rules! get {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn get(data: $data, id: String) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      match $class::get(&data, &id).await? {
        Some(item) => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&item)?)?),
        None => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::NOT_FOUND).body(format!("Item {} not found", id))?),
      }
    }
  };
}
pub(crate) use get;

macro_rules! update {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn update(data: $data, id: String, body: String) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      match $class::update(&data, &id, &body).await? {
        Some(item) => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&item)?)?),
        None => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::NOT_FOUND).body(format!("Item {} not found", id))?),
      }
    }
  };
}
pub(crate) use update;

macro_rules! delete {
  ( $class:ident, $class_sql:ident, $data:ident ) => {
    pub async fn delete(data: $data, id: String) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
      match $class::delete(&data, &id).await? {
        Some(item) => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&item)?)?),
        None => Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::NOT_FOUND).body(format!("Item {} not found", id))?),
      }
    }
  };
}
pub(crate) use delete;
