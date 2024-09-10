resource "azurerm_public_ip" "app" {
  name                = "${local.repository_name}-pip"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  allocation_method   = "Dynamic"
}

resource "azurerm_virtual_network" "app" {
  name                = "${local.repository_name}-vnet"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  address_space       = ["10.0.0.0/16"]
}

resource "azurerm_network_security_group" "app" {
  name                = "${local.repository_name}-nsg"
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
}

resource "azurerm_subnet" "app" {
  name                 = "${local.repository_name}-subnet"
  resource_group_name  = azurerm_resource_group.app.name
  virtual_network_name = azurerm_virtual_network.app.name

  address_prefixes  = ["10.0.1.0/24"]
  service_endpoints = ["Microsoft.Storage"]

  delegation {
    name = "fs"

    service_delegation {
      name = "Microsoft.DBforPostgreSQL/flexibleServers"
      actions = [
        "Microsoft.Network/virtualNetworks/subnets/join/action",
      ]
    }
  }
}

resource "azurerm_subnet_network_security_group_association" "app" {
  subnet_id                 = azurerm_subnet.app.id
  network_security_group_id = azurerm_network_security_group.app.id
}

resource "azurerm_private_dns_zone" "app" {
  name                = "${local.repository_name}.postgres.database.azure.com"
  resource_group_name = azurerm_resource_group.app.name
}

resource "azurerm_private_dns_zone_virtual_network_link" "app" {
  name                  = "lerpzVnetZone.com"
  private_dns_zone_name = azurerm_private_dns_zone.app.name
  virtual_network_id    = azurerm_virtual_network.app.id
  resource_group_name   = azurerm_resource_group.app.name
  depends_on            = [azurerm_subnet.app]
}
