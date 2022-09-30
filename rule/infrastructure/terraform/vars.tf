#--------------------------
# Required variables
# Do not add defaults
#--------------------------
variable "app_namespace" {
  type        = string
  description = "A namespace to be used on all resource names in this project"
}

variable "build_path" {
  type        = string
  description = "The path to the lambda function zip file"
}

#--------------------------
# Configurable variables
#--------------------------
variable "dns_zone" {
  type        = string
  description = "The dns zone to use for the domain name"
  default     = "dev.dogar.dev"
}

#--------------------------
# Interpolated values
#--------------------------
locals {
  project_prefix = "${var.app_namespace}-${terraform.workspace}"
  domain_name    = "${var.app_namespace}.${var.dns_zone}"
}
