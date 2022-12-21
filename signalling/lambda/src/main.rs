use lambda_http::RequestExt;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
  env_logger::init();
  lambda_http::run(lambda_http::service_fn(handle)).await?;
  Ok(())
}

async fn handle(event: lambda_http::Request) -> Result<lambda_http::Response<String>, lambda_http::Error> {
  match handler(event).await {
    Ok(r) => Ok(r),
    Err(e) => {
      log::error!("Error: {:#?}", e);
      let r = lambda_http::Response::builder()
      .status(lambda_http::http::StatusCode::INTERNAL_SERVER_ERROR)
      .body(e.to_string())?;
      Ok(r)
    }
  }
}

async fn handler(event: lambda_http::Request) -> Result<lambda_http::Response<String>, Box<dyn std::error::Error>> {
  if let lambda_http::request::RequestContext::ApiGatewayV1(context) = event.request_context() {
    log::debug!("Request context: {:#?}", context);
    log::debug!("Query string: {:#?}", event.query_string_parameters());
    let app_state = signalling::AppStateBuilder::default().filename(r"/mnt/efs/signalling.db3".to_string()).build()?;
    let path = context.resource_path.as_ref().ok_or("Unable to retrieve resource path")?;

    if regex::Regex::new(r"^/api/offer")?.is_match(path) {
      let request = signalling::lambda::Request::try_from(&event)?;
      let r = signalling::offer::apigw::handler(request, app_state).await?; Ok(r)

    } else if regex::Regex::new(r"^/api/answer")?.is_match(path) {
      let request = signalling::lambda::Request::try_from(&event)?;
      let r = signalling::answer::apigw::handler(request, app_state).await?; Ok(r)
 
    } else if regex::Regex::new(r"^/api/message")?.is_match(path) {
      let request = signalling::lambda::Request::try_from(&event)?;
      let r = signalling::message::apigw::handler(request, app_state).await?; Ok(r)

    } else if regex::Regex::new(r"^/api/health")?.is_match(path) {
      let r = lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body("Ok".to_string())?; Ok(r)

    } else {
      Err(format!("Uanble match path {}", path).into())
    }

  } else {
    Err("Request is not of type ApiGatewayV1".into())
  }
}
