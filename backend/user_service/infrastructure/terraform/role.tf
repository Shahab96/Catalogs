data "aws_iam_policy_document" "trust" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "this" {
  name               = local.project_prefix
  assume_role_policy = data.aws_iam_policy_document.trust.json
}

resource "aws_iam_role_policy_attachment" "this" {
  for_each = {
    basic  = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
    xray   = "arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess"
    lambda = aws_iam_policy.this.arn
  }

  role       = aws_iam_role.this.name
  policy_arn = each.value
}

data "aws_iam_policy_document" "this" {
  statement {
    effect = "Allow"
    actions = [
      "dynamodb:PutItem",
      "dynamodb:GetItem",
      "dynamodb:UpdateItem",
      "dynamodb:Query",
    ]
    resources = [
      aws_dynamodb_table.this.arn,
      "${aws_dynamodb_table.this.arn}/index/*",
    ]
  }

  statement {
    effect = "Allow"
    actions = [
      "secretsmanager:GetSecretValue",
    ]
    resources = [
      aws_secretsmanager_secret.this.arn,
      data.aws_secretsmanager_secret.this.arn,
    ]
  }
}

resource "aws_iam_policy" "this" {
  name   = local.project_prefix
  policy = data.aws_iam_policy_document.this.json
}
