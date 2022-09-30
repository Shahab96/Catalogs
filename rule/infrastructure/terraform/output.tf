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

output "zone" {
  value = aws_route53_zone.this
}
