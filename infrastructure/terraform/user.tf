module "user_service" {
  source = "../../user/infrastructure/terraform"

  app_namespace = var.user_namespace
  build_path    = "../../build/user/lambda.zip"
  dns_zone      = aws_route53_zone.this.name
}
