locals {
  domain_name = "${local.project_prefix}.${google_dns_managed_zone.this.dns_name}"
}

data "google_dns_managed_zone" "this" {
  name = "dogar-dev"
}

resource "google_dns_managed_zone" "this" {
  name     = terraform.workspace
  dns_name = "${terraform.workspace}.${data.google_dns_managed_zone.this.dns_name}"
}

resource "google_dns_record_set" "this" {
  name         = "${terraform.workspace}.${data.google_dns_managed_zone.this.dns_name}"
  managed_zone = data.google_dns_managed_zone.this.name
  type         = "NS"
  ttl          = 86400

  rrdatas = google_dns_managed_zone.this.name_servers
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
  domain_name                 = trimsuffix(local.domain_name, ".")
  domain_name_certificate_arn = aws_acm_certificate.this.arn

  # Routes and integrations
  integrations = {
    "ANY /{proxy+}" = {
      lambda_arn             = aws_lambda_function.this.arn
      payload_format_version = "2.0"
      timeout_milliseconds   = 30000
    }
  }
}

resource "aws_acm_certificate" "this" {
  domain_name       = trimsuffix(local.domain_name, ".")
  validation_method = "DNS"
}

resource "google_dns_record_set" "validation" {
  name         = tolist(aws_acm_certificate.this.domain_validation_options)[0].resource_record_name
  type         = tolist(aws_acm_certificate.this.domain_validation_options)[0].resource_record_type
  ttl          = 3600
  managed_zone = google_dns_managed_zone.this.name

  rrdatas = [
    tolist(aws_acm_certificate.this.domain_validation_options)[0].resource_record_value
  ]
}

resource "google_dns_record_set" "api" {
  name         = local.domain_name
  managed_zone = google_dns_managed_zone.this.name
  type         = "CNAME"
  ttl          = "60"

  rrdatas = [
    "${module.api_gateway.apigatewayv2_domain_name_target_domain_name}.",
  ]
}
