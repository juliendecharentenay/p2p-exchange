
data "aws_route53_zone" "domain" {
  name     = "${var.domain}."
}

data "aws_acm_certificate" "global" {
  provider    = aws.us_east_1
  domain      = "${var.domain}"
  types       = ["AMAZON_ISSUED"]
}

/*
 * Regional SSL certificate
 */
resource "aws_acm_certificate" "regional" {
  domain_name       = "${var.domain}"
  subject_alternative_names = [ "*.${var.domain}" ]
  validation_method = "DNS"
  tags              = local.all_tags
}


resource "aws_route53_record" "regional" {
  for_each = {
    for dvo in aws_acm_certificate.regional.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }
  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.domain.zone_id
}

resource "aws_acm_certificate_validation" "regional" {
  certificate_arn         = aws_acm_certificate.regional.arn
  validation_record_fqdns = [for record in aws_route53_record.regional : record.fqdn]
}

