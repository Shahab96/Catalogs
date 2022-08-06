resource "aws_lambda_function" "this" {
  function_name = local.project_prefix
  filename      = "../../dist/handler"
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
