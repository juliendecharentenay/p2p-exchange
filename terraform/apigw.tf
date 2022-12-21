
resource "aws_api_gateway_rest_api" "app" {
  name = "${local.application} API Gateway-${var.environment}"
  disable_execute_api_endpoint = true
  endpoint_configuration { types = ["REGIONAL"] }
  tags = local.all_tags
}

resource "aws_ssm_parameter" "app" {
  name = "/${local.application}/${var.environment}/apigateway/id"
  type = "String"
  value = "${aws_api_gateway_rest_api.app.id}"
  tags  = local.all_tags
}

/*
 * Routes/resources
 */
// Route: /api
resource "aws_api_gateway_resource" "api" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_rest_api.app.root_resource_id
  path_part   = "api"
}

// Route: /api/health
resource "aws_api_gateway_resource" "api_health" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api.id
  path_part   = "health"
}

// Route: /api/answer
resource "aws_api_gateway_resource" "api_answer" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api.id
  path_part   = "answer"
}

// Route: /api/answer/{key}
resource "aws_api_gateway_resource" "api_answer_key" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api_answer.id
  path_part   = "{key}"
}

// Route: /api/message
resource "aws_api_gateway_resource" "api_message" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api.id
  path_part   = "message"
}

// Route: /api/offer
resource "aws_api_gateway_resource" "api_offer" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api.id
  path_part   = "offer"
}

// Route: /api/offer/{key}
resource "aws_api_gateway_resource" "api_offer_key" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  parent_id   = aws_api_gateway_resource.api_offer.id
  path_part   = "{key}"
}

/*
 * Methods
 */
locals {
  methods = [
    { 
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_answer.id
      request_parameters = { "method.request.querystring.offer_id" = true }
    },
    { 
      http_method = "POST"
      resource_id = aws_api_gateway_resource.api_answer.id
      request_parameters = {}
    },
    { 
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_answer_key.id
      request_parameters = {}
    },
    { 
      http_method = "PUT"
      resource_id = aws_api_gateway_resource.api_answer_key.id
      request_parameters = {}
    },
    { 
      http_method = "DELETE"
      resource_id = aws_api_gateway_resource.api_answer_key.id
      request_parameters = {}
    },

    { 
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_message.id
      request_parameters = {"method.request.querystring.originator_id" = true}
    },
    { 
      http_method = "POST"
      resource_id = aws_api_gateway_resource.api_message.id
      request_parameters = {}
    },

    {
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_offer.id
      request_parameters = {}
    },
    {
      http_method = "POST"
      resource_id = aws_api_gateway_resource.api_offer.id
      request_parameters = {}
    },
    {
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_offer_key.id
      request_parameters = {}
    },
    {
      http_method = "PUT"
      resource_id = aws_api_gateway_resource.api_offer_key.id
      request_parameters = {}
    },
    {
      http_method = "DELETE"
      resource_id = aws_api_gateway_resource.api_offer_key.id
      request_parameters = {}
    },

    { 
      http_method = "GET"
      resource_id = aws_api_gateway_resource.api_health.id
      request_parameters = {}
    }
  ]
}

resource "aws_api_gateway_method" "methods" {
  for_each      = { for idx, v in local.methods: idx=>v}
  rest_api_id   = aws_api_gateway_rest_api.app.id
  resource_id   = each.value.resource_id
  http_method   = each.value.http_method
  authorization = "NONE"
  request_parameters = each.value.request_parameters
}
resource "aws_api_gateway_integration" "integrations" {
  for_each     = aws_api_gateway_method.methods
  rest_api_id  = aws_api_gateway_rest_api.app.id
  resource_id  = each.value.resource_id
  http_method  = each.value.http_method
  type         = "AWS_PROXY"
  integration_http_method = "POST"
  uri          = aws_lambda_function.app.invoke_arn
}

/*
 * Stage and deployment
 */
resource "aws_api_gateway_deployment" "app" {
  rest_api_id = aws_api_gateway_rest_api.app.id
  triggers = { for idx, v in aws_api_gateway_method.methods: idx => v.id }
  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "app" {
  deployment_id = aws_api_gateway_deployment.app.id
  rest_api_id   = aws_api_gateway_rest_api.app.id
  stage_name    = "api"
}

/*
 * Route53 DNS entry
 */
resource "aws_api_gateway_domain_name" "app" {
  domain_name     = "${local.application}-${var.environment}-api.${var.domain}"
  regional_certificate_arn = aws_acm_certificate.regional.arn
  security_policy = "TLS_1_2"
  endpoint_configuration { types = ["REGIONAL"] }
  tags            = local.all_tags
}

resource "aws_api_gateway_base_path_mapping" "app" {
  api_id      = aws_api_gateway_rest_api.app.id
  stage_name  = aws_api_gateway_stage.app.stage_name
  domain_name = aws_api_gateway_domain_name.app.domain_name
}

resource "aws_route53_record" "app" {
  zone_id = data.aws_route53_zone.domain.zone_id
  name    = aws_api_gateway_domain_name.app.domain_name
  type    = "A"
  alias {
    name    = aws_api_gateway_domain_name.app.regional_domain_name
    zone_id = aws_api_gateway_domain_name.app.regional_zone_id
    evaluate_target_health = true
  }
}

