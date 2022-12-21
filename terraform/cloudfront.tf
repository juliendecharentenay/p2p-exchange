/*
 * Cloudfront CDN
 */
locals {
  s3_origin = "www_s3"
  apigw_origin = "apigw"
}

data "aws_wafv2_web_acl" "cloudfront" {
  provider = aws.us_east_1
  name     = "CloudFrontBaseWAF"
  scope    = "CLOUDFRONT"
}

resource "aws_cloudfront_origin_access_identity" "www" { }

resource "aws_cloudfront_distribution" "www" {
  enabled             = true
  default_root_object = "index.html"
  aliases = var.subdomains
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
  price_class        = "PriceClass_100"
  viewer_certificate {
    acm_certificate_arn            = data.aws_acm_certificate.global.arn
    cloudfront_default_certificate = false
    ssl_support_method             = "sni-only"
    minimum_protocol_version       = "TLSv1.2_2021"
  }

  web_acl_id = data.aws_wafv2_web_acl.cloudfront.arn

  // [default] S3 origin
  origin {
    domain_name = aws_s3_bucket.www.bucket_regional_domain_name
    origin_id   = local.s3_origin
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.www.cloudfront_access_identity_path
    }
  }

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = local.s3_origin
    viewer_protocol_policy = "redirect-to-https"
    forwarded_values {
      query_string = false
      headers      = []
      cookies {
        forward    = "none"
      }
    }
    min_ttl     = 0
    default_ttl = 86400    # 24 hours
    max_ttl     = 2592000  # 30 days
  }

 // API Gateway origin
  origin {
    origin_id   = local.apigw_origin
    domain_name = aws_api_gateway_domain_name.app.domain_name
    custom_origin_config {
      http_port = "80"
      https_port = "443"
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }
  ordered_cache_behavior {
    target_origin_id = local.apigw_origin
    path_pattern     = "/api/*"
    allowed_methods  = ["HEAD", "DELETE", "POST", "GET", "OPTIONS", "PUT", "PATCH"]
    cached_methods   = ["GET", "HEAD"]
    viewer_protocol_policy = "redirect-to-https"
    forwarded_values {
      query_string   = true
      headers        = ["Authorization"]
      cookies { forward = "none" }
    }
    min_ttl          = 0
    default_ttl      = 0
    max_ttl          = 0
  }

  tags = local.all_tags
}

resource "aws_ssm_parameter" "cloudfront_id" {
  name = "/${local.application}/${var.environment}/cloudfront/id"
  type = "String"
  value = "${aws_cloudfront_distribution.www.id}"
  tags  = local.all_tags
}

/*
 * Route53 DNS entry
 */
resource "aws_route53_record" "www" {
  for_each = toset(var.subdomains)
  zone_id = data.aws_route53_zone.domain.zone_id
  name    = each.key
  type    = "A"
  alias {
    name    = aws_cloudfront_distribution.www.domain_name
    zone_id = aws_cloudfront_distribution.www.hosted_zone_id
    evaluate_target_health = true
  }
}

