module "extraction_service" {
  source = "../../extraction/infrastructure/terraform"

  app_namespace = var.extraction_namespace
  build_path    = "../../build/extraction/lambda.zip"
  dns_zone      = aws_route53_zone.this.name
}
