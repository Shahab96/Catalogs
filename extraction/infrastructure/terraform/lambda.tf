resource "aws_lambda_function" "this" {
  function_name = local.project_prefix
  filename      = var.build_path
  description   = filesha256(var.build_path)
  handler       = "bootstrap"
  runtime       = "provided.al2"
  memory_size   = 128
  architectures = ["arm64"]
  timeout       = 900
  role          = aws_iam_role.this.arn

  environment {
    variables = {
      STAGE = terraform.workspace
    }
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
