module "extraction_service" {
  source = "../../extraction/infrastructure/terraform"
 
  app_namespace = var.extraction_namespace
  build_path = "../../build/extraction/lambda.zip"
  access_token_secret = module.user_service.access_token.id
}
