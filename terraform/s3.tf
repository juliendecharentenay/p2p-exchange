resource "aws_s3_bucket" "www" {
  bucket        = "${local.application}-www-${var.environment}"
  acl           = "private"
  force_destroy = true
  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }

  tags = merge(local.all_tags, { Name = "${local.application}-www-${var.environment}" })
}

resource "aws_s3_bucket_policy" "www" {
  bucket = aws_s3_bucket.www.id
  policy = data.aws_iam_policy_document.www.json
}

data "aws_iam_policy_document" "www" {
  statement {
    principals {
      type = "AWS"
      identifiers = [ aws_cloudfront_origin_access_identity.www.iam_arn ]
    }
    actions   = [ "s3:GetObject", ]
    resources = [ "${aws_s3_bucket.www.arn}/*", ]
  }
}


