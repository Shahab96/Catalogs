resource "aws_dynamodb_table" "this" {
  name         = local.project_prefix
  billing_mode = "PAY_PER_REQUEST"

  hash_key  = "pk"

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "gsi_uuid"
    type = "S"
  }

  global_secondary_index {
    name            = "uuid-index"
    hash_key        = "gsi_uuid"
    projection_type = "ALL"
  }

  server_side_encryption {
    enabled = true
  }
}

