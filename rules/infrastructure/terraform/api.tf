resource "aws_apigatewayv2_api" "this" {
  name          = local.project_prefix
  protocol_type = "HTTP"
  body          = data.template_file.this.rendered
}

resource "aws_apigatewayv2_integration" "this" {
  api_id                    = aws_apigatewayv2_api.this.id
  integration_type          = "AWS"
  connection_type           = "INTERNET"
  content_handling_strategy = "CONVERT_TO_TEXT"
  integration_method        = "POST"
  integration_uri           = aws_lambda_function.this.invoke_arn
  passthrough_behavior      = "WHEN_NO_MATCH"
}

resource "aws_apigatewayv2_route" "this" {
  api_id    = aws_apigatewayv2_api.this.id
  route_key = "ANY /{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.this.id}"
}

resource "aws_apigatewayv2_stage" "this" {
  api_id      = aws_apigatewayv2_api.this.id
  name        = terraform.workspace
  auto_deploy = true
}
