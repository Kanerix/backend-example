variable "deploy_env" {
  description = "Deployment environment"
  type        = string
  default     = "stag"
}

variable "database_username" {
  description = "The username for the Postgres database."
  type        = string
  sensitive   = true
}

variable "database_password" {
  description = "The password for the Postgres database."
  type        = string
  sensitive   = true
}
