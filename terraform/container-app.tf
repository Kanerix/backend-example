resource "azurerm_container_app" "app" {
  name                         = "${local.repository_name}-ca"
  resource_group_name          = azurerm_resource_group.app.name
  container_app_environment_id = azurerm_container_app_environment.app.id
  revision_mode                = "Single"

  secret {
    name  = "database-url"
    value = "postgresql://${var.database_username}:${var.database_password}@${data.azurerm_postgresql_flexible_server.primary.fqdn}/${local.repository_name}-${var.deploy_env}"
  }

  secret {
    name  = "pwd-secret"
    value = random_string.pwd_secret.result
  }

  secret {
    name  = "github-token"
    value = var.registry_password
  }

  template {
    container {
      name   = "${local.repository_name}-${var.deploy_env}"
      image  = "ghcr.io/lerpz-com/${local.repository_name}:${var.deploy_env}"
      cpu    = 0.25
      memory = "0.5Gi"

      env {
        name  = "ENV"
        value = var.deploy_env
      }

      env {
        name  = "API_ORIGIN"
        value = "https://api.lerpz.com"
      }

      env {
        name  = "RUST_LOG"
        value = "info"
      }

      env {
        name        = "DATABASE_URL"
        secret_name = "database-url"
      }

      env {
        name        = "PWD_SECRET"
        secret_name = "pwd-secret"
      }
    }
  }

  registry {
    server               = "ghcr.io"
    username             = "Kanerix"
    password_secret_name = "github-token"
  }
}

resource "azurerm_container_app_environment" "app" {
  name                       = "${local.repository_name}-cae"
  location                   = azurerm_resource_group.app.location
  resource_group_name        = azurerm_resource_group.app.name
  log_analytics_workspace_id = azurerm_log_analytics_workspace.app.id
}
