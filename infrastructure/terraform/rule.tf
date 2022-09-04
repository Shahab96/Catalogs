module "rule_service" {
  source = "../../rule/infrastructure/terraform"

  app_namespace = var.rule_namespace
}
