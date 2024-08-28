resource "random_string" "pwd_secret" {
  length  = 32
  special = true
}

resource "github_actions_environment_variable" "env" {
  repository    = data.github_repository.primary.name
  environment   = var.deploy_env
  variable_name = "ENV"
  value         = var.deploy_env
}

resource "github_actions_environment_secret" "database_url" {
  repository      = data.github_repository.primary.name
  environment     = var.deploy_env
  secret_name     = "DATABASE_URL"
  plaintext_value = "postgresql://lerpz:Password1234!@${azurerm_postgresql_flexible_server.server.fqdn}:5432/primary"
}

resource "github_actions_environment_variable" "api_origin" {
  repository    = data.github_repository.primary.name
  environment   = var.deploy_env
  variable_name = "API_ORIGIN"
  value         = "https://api.lerpz.com"
}

resource "github_actions_environment_secret" "pwd_secret" {
  repository      = data.github_repository.primary.name
  environment     = var.deploy_env
  secret_name     = "PWD_SECRET"
  plaintext_value = random_string.pwd_secret.result
}
