data "template_file" "this" {
  template = file("api.yaml")
  vars = {
    lambda_arn = "${aws_lambda_function.this.arn}"
  }
}

resource "aws_apigatewayv2_api" "this" {
  name          = local.project_prefix
  protocol_type = "HTTP"
  body          = data.template_file.this.rendered
}

resource "aws_apigatewayv2_stage" "this" {
  api_id      = aws_apigatewayv2_api.this.id
  name        = terraform.workspace
  auto_deploy = true
}
