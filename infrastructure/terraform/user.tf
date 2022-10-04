module "user_service" {
  source = "../../backend/user_service/infrastructure/terraform"

  app_namespace = var.user_namespace
  build_path    = "../../build/user/lambda.zip"
  dns_zone      = aws_route53_zone.this.name
}
