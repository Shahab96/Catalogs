module "rule_service" {
  source = "../../backend/rule_service/infrastructure/terraform"

  app_namespace = var.rule_namespace
  build_path    = "../../build/rule/lambda.zip"
  dns_zone      = aws_route53_zone.this.name
}
