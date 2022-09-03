resource "random_password" "this" {
  count = 2

  length = 64
  special = true
}

resource "aws_secretsmanager_secret" "this" {
  for_each = toset(["access", "refresh"])

  name = "${local.project_prefix}-${each.value}-token"
}

resource "aws_secretsmanager_secret_version" "this" {
  for_each = {
    access = random_password.this[0].result
    refresh = random_password.this[1].result
  }

  secret_id     = aws_secretsmanager_secret.this[each.key].id
  secret_string = each.value
}
