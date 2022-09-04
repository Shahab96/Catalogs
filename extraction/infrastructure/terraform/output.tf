output "api_gateway" {
  value = module.api_gateway
}

output "lambda" {
  value = aws_lambda_function.this
}

output "role" {
  value = aws_iam_role.this
}

output "domain_name" {
  value = local.domain_name
}
