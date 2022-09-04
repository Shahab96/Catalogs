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

#--------------------------
# Interpolated values
#--------------------------
locals {
  project_prefix = "${var.app_namespace}-${terraform.workspace}"
}
