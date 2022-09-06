#--------------------------
# Required variables
# Do not add defaults
#--------------------------
variable "aws_region" {
  type = string
  description = "The AWS region to deploy to"
}

variable "gcp_region" {
  type = string
  description = "The GCP region to deploy to"
}

#--------------------------
# Configurable variables
#--------------------------
variable "app_namespace" {
  type = string
  description = "The namespace to use for the application" 
}

variable "user_namespace" {
  type = string
  description = "The namespace to use for the user service"
}

variable "rule_namespace" {
  type = string
  description = "The namespace to use for the rule service"
}

variable "extraction_namespace" {
  type = string
  description = "The namespace to use for the extraction service"
}

#--------------------------
# Interpolated values
#--------------------------
locals {
  project_prefix = "${var.app_namespace}-${terraform.workspace}"
}
