variable "deploy_env" {
  description = "Deployment environment"
  type        = string
  default     = "stag"
}

variable "registry_password" {
  description = "Access token for ghcr.io"
  type        = string
  sensitive   = true
}

variable "database_password" {
  description = "Database password"
  type        = string
  sensitive   = true
}

variable "database_username" {
  description = "Database username"
  type        = string
  sensitive   = true
}
