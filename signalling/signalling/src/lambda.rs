use lambda_http::{
  aws_lambda_events::apigw::ApiGatewayProxyRequestContext,
  http::method::Method,
  request::RequestContext,
  RequestExt
};

pub enum Request {
  Post   { body: String },
  List   { query: query_map::QueryMap },
  Get    { key: String },
  Update { key: String, body: String },
  Delete { key: String },
  Count,
}

impl TryFrom<&lambda_http::Request> for Request {
  type Error = Box<dyn std::error::Error>;

  fn try_from(event: &lambda_http::Request) -> Result<Self, Self::Error> {
    if let RequestContext::ApiGatewayV1(ApiGatewayProxyRequestContext { http_method, resource_path, .. }) = event.request_context() {
      let params = event.path_parameters();
      let body = match event.body() {
          lambda_http::Body::Text(body) => Some(body),
          _ => None,
      };
      match http_method {
        Method::GET if resource_path.unwrap_or_else(|| "".to_string()).ends_with("/count") => {
          Ok(Request::Count)
        },
        Method::GET if params.first("key").is_some() => {
          let key = urlencoding::decode(params.first("key").unwrap())?.to_string();
          Ok(Request::Get { key })
        },
        Method::GET => {
          Ok(Request::List { query: event.query_string_parameters() })
        },
        Method::POST if body.is_some() => {
          Ok(Request::Post { body: body.unwrap().clone() })
        },
        Method::DELETE if params.first("key").is_some() => {
          let key = urlencoding::decode(params.first("key").unwrap())?.to_string();
          Ok(Request::Delete { key })
        },
        Method::PATCH if params.first("key").is_some() && body.is_some() => {
          let key = urlencoding::decode(params.first("key").unwrap())?.to_string();
          Ok(Request::Update { key, body: body.unwrap().clone() })
        },
        _ => Err("Request is not supported".into()),
      }
    } else {
      Err("Unable to process request event".into())
    }
  }
}
