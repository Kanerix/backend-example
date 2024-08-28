resource "azurerm_network_security_group" "app" {
  name                = "${local.repository_name}-nsg"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
}

resource "azurerm_virtual_network" "app" {
  name                = "${local.repository_name}-vnet"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  address_space       = ["10.0.0.0/24"]

  subnet {
    name           = var.deploy_env
    address_prefix = "10.0.1.0/24"
    security_group = azurerm_network_security_group.app.id
  }
}
