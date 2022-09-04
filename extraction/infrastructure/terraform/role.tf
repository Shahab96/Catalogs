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
  for_each = {
    basic  = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
    xray   = "arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess"
  }

  role       = aws_iam_role.this.name
  policy_arn = each.value
}
