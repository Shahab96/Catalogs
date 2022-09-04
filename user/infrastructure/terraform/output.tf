output "api_gateway" {
  value = module.api_gateway
}

output "lambda" {
  value = aws_lambda_function.this
}

output "table" {
  value = aws_dynamodb_table.this
}

output "role" {
  value = aws_iam_role.this
}

output "domain_name" {
  value = local.domain_name
}

output "access_token" {
  value = aws_secretsmanager_secret_version.this["access"]
}

output "refresh_token" {
  value = aws_secretsmanager_secret_version.this["refresh"]
}