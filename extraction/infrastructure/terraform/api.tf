resource "aws_route53_zone" "this" {
  name = local.domain_name
}

module "api_gateway" {
  source = "terraform-aws-modules/apigateway-v2/aws"

  name                         = local.project_prefix
  create_vpc_link              = false
  disable_execute_api_endpoint = true

  cors_configuration = {
    allow_headers = [
      "content-type",
      "x-amz-date",
      "authorization",
      "x-api-key",
      "x-amz-security-token",
      "x-amz-user-agent",
    ]
    allow_methods = ["*"]
    allow_origins = ["*"]
  }

  # Custom domain
  domain_name                 = aws_route53_zone.this.name
  domain_name_certificate_arn = module.acm.acm_certificate_arn

  # Routes and integrations
  integrations = {
    "ANY /{proxy+}" = {
      lambda_arn             = aws_lambda_function.this.arn
      payload_format_version = "2.0"
      timeout_milliseconds   = 30000
    }
  }
}

module "acm" {
  source  = "terraform-aws-modules/acm/aws"
  version = "~> 3.0"

  domain_name = aws_route53_zone.this.name
  zone_id     = aws_route53_zone.this.zone_id

  wait_for_validation = true
}

resource "aws_route53_record" "this" {
  zone_id = aws_route53_zone.this.zone_id
  name    = aws_route53_zone.this.name
  type    = "A"

  alias {
    name                   = module.api_gateway.apigatewayv2_domain_name_target_domain_name
    zone_id                = module.api_gateway.apigatewayv2_domain_name_hosted_zone_id
    evaluate_target_health = false
  }
}

