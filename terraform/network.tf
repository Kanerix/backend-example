data "azurerm_resource_group" "infrastructure_network" {
  name = "lerpz-infrastructure-network"
}

data "azurerm_virtual_network" "primary" {
  name                = "lerpz-infrastructure-vnet"
  resource_group_name = data.azurerm_resource_group.infrastructure_network.name
}

resource "azurerm_subnet" "app" {
  name                 = "${local.repository_name}-${var.deploy_env}"
  resource_group_name  = data.azurerm_resource_group.infrastructure_network.name
  virtual_network_name = data.azurerm_virtual_network.primary.name
  address_prefixes     = [var.deploy_env == "prod" ? "10.2.0.0/16" : "10.3.0.0/16"]
}
