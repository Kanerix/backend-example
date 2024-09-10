resource "azurerm_postgresql_flexible_server" "app" {
  name                = "${local.repository_name}-pgs"
  resource_group_name = azurerm_resource_group.app.name
  location            = azurerm_resource_group.app.location

  administrator_login    = var.database_username
  administrator_password = var.database_password
  sku_name               = "B_Standard_B1ms"

  storage_mb            = 32768
  version               = "12"
  auto_grow_enabled     = false
  backup_retention_days = 7

  delegated_subnet_id           = azurerm_subnet.app.id
  private_dns_zone_id           = azurerm_private_dns_zone.app.id
  public_network_access_enabled = false

  identity {
    type = "UserAssigned"

    identity_ids = [
      azurerm_user_assigned_identity.app.id
    ]
  }

  lifecycle {
    prevent_destroy = true
    ignore_changes = [
      zone
    ]
  }
}
