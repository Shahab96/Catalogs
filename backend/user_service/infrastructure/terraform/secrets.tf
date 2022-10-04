resource "random_password" "this" {
  count = 2

  length = 64
  special = true
}

resource "aws_secretsmanager_secret" "this" {
  name = "${local.project_prefix}-rsa-key"
}

data "aws_secretsmanager_secret" "this" {
  name = "google-oauth-credentials"
}
