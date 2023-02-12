# Response policy to override response headers
# 
resource "aws_cloudfront_response_headers_policy" "default" { 
  name = "${local.application}-${var.environment}"
  comment = "Add security headers" 
  security_headers_config { 
    content_security_policy { 
      content_security_policy = "default-src https:" 
      override = true 
    } 
    content_type_options { 
      override = true 
    } 
    frame_options { 
      frame_option = "SAMEORIGIN" 
      override = true 
    } 
    referrer_policy { 
      referrer_policy = "strict-origin-when-cross-origin" 
      override = true 
    } 
    strict_transport_security { 
      access_control_max_age_sec = 31536000 
      override = true 
    } 
    xss_protection { 
      protection = true 
      mode_block = true 
      override = true 
    } 
  } 
  custom_headers_config { 
    items { 
      header = "Permissions-Policy" 
      override = true 
      value = "interest-cohort=()" 
    } 
  } 
}
