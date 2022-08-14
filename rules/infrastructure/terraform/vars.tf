#--------------------------
# Required variables
# Do not add defaults
#--------------------------
variable "app_namespace" {
  type        = string
  description = "A namespace to be used on all resource names in this project"
}

#--------------------------
# Configurable variables
#--------------------------
variable "region" {
  type        = string
  description = "The region to deploy in AWS"
  default     = "us-west-2"
}

variable "gcp_region" {
  type        = string
  description = "The region for resources in GCP"
  default     = "us-central1"
}

#--------------------------
# Interpolated values
#--------------------------
locals {
  project_prefix = "${var.app_namespace}-${terraform.workspace}"
}
