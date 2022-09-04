terraform {
  backend "s3" {
    encrypt = true
  }
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    google = {
      source  = "hashicorp/google"
      version = "4.31.0"
    }
  }
}

# Configure the AWS Provider
provider "aws" {
  region = var.aws_region
}

provider "google" {
  region  = var.gcp_region
}

data "aws_region" "current" {}
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}

