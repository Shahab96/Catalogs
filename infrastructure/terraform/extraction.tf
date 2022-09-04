module "extraction_service" {
  source = "../../extraction/infrastructure/terraform"
 
  app_namespace = var.extraction_namespace
}
