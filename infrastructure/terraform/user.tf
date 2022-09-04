module "user_service" {
  source = "../../user/infrastructure/terraform"
 
  app_namespace = var.user_namespace
  build_path = "./build/user.zip"
}
