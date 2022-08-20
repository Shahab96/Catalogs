locals {
  domain_name = trimsuffix("${local.project_prefix}.${terraform.workspace}.${data.google_dns_managed_zone.this.dns_name}", ".")
}

data "google_dns_managed_zone" "this" {
  name = "dogar-dev"
}

resource "aws_route53_zone" "this" {
  name = local.domain_name
}

resource "google_dns_record_set" "this" {
  name = "${local.domain_name}."
  managed_zone = data.google_dns_managed_zone.this.name
  type = "NS"
  ttl = 86400
  rrdatas = [for name_server in aws_route53_zone.this.name_servers : "${name_server}."]
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
  domain_name                 = local.domain_name
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

  domain_name  = local.domain_name
  zone_id      = aws_route53_zone.this.zone_id

  wait_for_validation = true
}

resource "aws_route53_record" "this" {
  zone_id = aws_route53_zone.this.zone_id
  name = local.domain_name
  type = "A"

  alias {
    name    = module.api_gateway.apigatewayv2_domain_name_target_domain_name
    zone_id = module.api_gateway.apigatewayv2_domain_name_hosted_zone_id
    evaluate_target_health = false
  }
}
