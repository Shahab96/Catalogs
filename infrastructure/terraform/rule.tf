module "rule_service" {
  source = "../../rule/infrastructure/terraform"

  app_namespace = var.rule_namespace
  build_path = "../../build/rule/lambda.zip"
}
