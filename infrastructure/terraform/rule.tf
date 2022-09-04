module "rule_service" {
  source = "../../rule/infrastructure/terraform"

  app_namespace = var.rule_namespace
  build_path = "../../build/rule/lambda.zip"
  access_token_secret = module.user_service.access_token.id
}
