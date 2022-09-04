terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
    }
    google = {
      source  = "hashicorp/google"
    }
  }
}

data "aws_region" "current" {}
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}
