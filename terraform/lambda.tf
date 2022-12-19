/*
 * Lambda for API Gateway
 */
resource "aws_lambda_function" "app" {
  function_name = "${local.application}-${var.environment}"
  role          = aws_iam_role.lambda.arn
  memory_size   = 128
  timeout       = 5
  handler       = "bootstrap"
  runtime       = "provided"
  package_type  = "Zip"
  filename      = "files/empty.zip"

  // Permanent file storage
  file_system_config {
    arn = aws_efs_access_point.lambda.arn
    local_mount_path = "/mnt/efs"
  }

  vpc_config {
    subnet_ids         = [ aws_subnet.privateA.id ]
    security_group_ids = [ aws_security_group.lambda.id ]
  }

  environment {
    variables = {
      RUST_LOG = "debug"
    }
  }

  depends_on = [ aws_efs_mount_target.lambda ]

  tags          = local.all_tags
}

resource "aws_lambda_permission" "app" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.app.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_api_gateway_rest_api.app.execution_arn}/*/*/*"
}

resource "aws_iam_role" "lambda" {
  name = "${local.application}-api-${var.environment}"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
       {
           Effect = "Allow"
           Action = "sts:AssumeRole",
           Principal = { Service = "lambda.amazonaws.com" }
       }
    ]
  })
  inline_policy {
    name = "AccessToLogs"
    policy = jsonencode({
      Version = "2012-10-17"
      Statement = [
      {
           Effect = "Allow",
           Action = [
             "logs:CreateLogStream",
             "logs:CreateLogGroup",
             "logs:PutLogEvents"
           ],
           Resource = "*"
      }]
    })
  }
  inline_policy {
    name = "NetworkInterface"
    policy = jsonencode({
      Version = "2012-10-17"
      Statement = [
      {
        Effect = "Allow",
        Action = [
          "ec2:CreateNetworkInterface",
          "ec2:DescribeNetworkInterfaces",
        ],
        Resource = "*"
      },
      {
        Effect = "Allow",
        Action = [
          "ec2:DeleteNetworkInterface",
        ],
        Resource = "*"
      }]
    })
  }
  inline_policy {
    name = "EFS"
    policy = jsonencode({
      Version = "2012-10-17"
      Statement = [
      {
        Effect = "Allow",
        Action = [
          "elasticfilesystem:ClientMount",
          "elasticfilesystem:ClientRootAccess",
          "elasticfilesystem:ClientWrite",
          "elasticfilesystem:DescribeMountTargets"
        ],
        Resource = "*"
      }]
    })
  }

  tags = local.all_tags
}

/*
 * EFS
 */
resource "aws_efs_file_system" "lambda" {
  encrypted = true
  tags = merge(local.all_tags, { Name = "${local.application}-efs-${var.environment}" })
}

resource "aws_efs_mount_target" "lambda" {
  file_system_id = aws_efs_file_system.lambda.id
  subnet_id      = aws_subnet.privateA.id
  security_groups = [ aws_security_group.efs.id ]
}

resource "aws_efs_access_point" "lambda" {
  file_system_id = aws_efs_file_system.lambda.id
  root_directory {
    path = "/efs"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "777"
    }
  }
  posix_user {
    gid = 1000
    uid = 1000
  }
  tags = merge(local.all_tags, { Name = "${local.application}-efs-${var.environment}" })
}

/*
 * Security group 
 *      Lambda -> EFS
 */
resource "aws_security_group" "lambda" {
  name        = "${local.application}-lambda-${var.environment}"
  description = "Security group assigned to lambda"
  vpc_id      = aws_vpc.main.id
  tags        = local.all_tags
}
resource "aws_security_group_rule" "lambda_to_efs" {
  type        = "egress"
  from_port   = 2049
  to_port     = 2049
  protocol    = "tcp"
  security_group_id = aws_security_group.lambda.id
  source_security_group_id = aws_security_group.efs.id
}

resource "aws_security_group" "efs" {
  name        = "${local.application}-efs-${var.environment}"
  description = "Security group assigned to efs"
  vpc_id      = aws_vpc.main.id
  tags        = local.all_tags
}
resource "aws_security_group_rule" "efs_from_lambda" {
  type        = "ingress"
  from_port   = 2049
  to_port     = 2049
  protocol    = "tcp"
  security_group_id = aws_security_group.efs.id
  source_security_group_id = aws_security_group.lambda.id
}


