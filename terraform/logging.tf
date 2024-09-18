resource "azurerm_log_analytics_workspace" "app" {
  name                = "${local.repository_name}-law"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  sku                 = "PerGB2018"
  retention_in_days   = 31
}
