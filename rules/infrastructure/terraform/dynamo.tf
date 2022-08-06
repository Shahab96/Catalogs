resource "aws_dynamodb_table" "this" {
  name         = local.project_prefix
  billing_mode = "PAY_PER_REQUEST"

  hash_key  = "id"
  range_key = "sort_key"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "sort_key"
    type = "S"
  }

  server_side_encryption {
    enabled = true
  }
}

