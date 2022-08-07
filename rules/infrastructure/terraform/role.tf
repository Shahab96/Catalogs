data "aws_iam_policy_document" "assume" {
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
  assume_role_policy = data.aws_iam_policy_document.assume.json
}

resource "aws_iam_role_policy_attachment" "this" {
  role       = aws_iam_role.this.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "aws_iam_policy_document" "permissions" {
  statement {
    effect = "Allow"
    actions = [
      "dynamodb:GetItem",
      "dynamodb:PutItem",
      "logs:DescribeLogGroups"
    ]
    resources = [
      aws_dynamodb_table.this.arn,
      aws_cloudwatch_log_group.this.name
    ]
  }
}

resource "aws_iam_policy" "this" {
  name   = local.project_prefix
  policy = data.aws_iam_policy_document.permissions.json
}
