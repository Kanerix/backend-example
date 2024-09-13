resource "azurerm_container_app" "app" {
  name                         = "${local.repository_name}-ca"
  resource_group_name          = azurerm_resource_group.app.name
  container_app_environment_id = azurerm_container_app_environment.app.id
  revision_mode                = "Single"

  template {
    container {
      name   = "container-app-${var.deploy_env}"
      image  = "ghcr.io/lerpz-com/lerpz-backend:prod"
      cpu    = 0.25
      memory = "0.2Gi"
    }
  }
}

resource "azurerm_container_app_environment" "app" {
  name                       = "${local.repository_name}-cae"
  location                   = azurerm_resource_group.app.location
  resource_group_name        = azurerm_resource_group.app.name
  log_analytics_workspace_id = azurerm_log_analytics_workspace.example.id
}