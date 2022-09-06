resource "aws_lambda_function" "this" {
  function_name = local.project_prefix
  filename      = var.build_path
  description   = filesha256(var.build_path)
  handler       = "bootstrap"
  runtime       = "provided.al2"
  architectures = ["arm64"]
  memory_size   = 1024
  timeout       = 30
  role          = aws_iam_role.this.arn

  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.this.name
      RSA_KEY_SECRET = aws_secretsmanager_secret.this.arn
      DOMAIN_NAME = "https://${local.domain_name}/"
    }
  }

  tracing_config {
    mode = "Active"
  }
}

resource "aws_lambda_permission" "this" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.this.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${module.api_gateway.apigatewayv2_api_execution_arn}/*/*/*"
}

resource "aws_cloudwatch_log_group" "this" {
  name              = "/aws/lambda/${local.project_prefix}"
  retention_in_days = 30
}
