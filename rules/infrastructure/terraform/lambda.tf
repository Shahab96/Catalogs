resource "aws_lambda_function" "this" {
  function_name = local.project_prefix
  filename      = "../../dist/handler.zip"
  handler       = "main"
  runtime       = "go1.x"
  memory_size   = 128
  timeout       = 30
  role          = aws_iam_role.this.arn

  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.this.name
    }
  }
}

resource "aws_lambda_permission" "this" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.this.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = aws_apigatewayv2_api.this.execution_arn
}
