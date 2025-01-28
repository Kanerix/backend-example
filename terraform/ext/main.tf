data "azurerm_client_config" "current" {}

data "azurerm_subscription" "current" {}

resource "azurerm_resource_group" "ext" {
  name     = "${local.repository_name}-ext"
  location = local.location
}
