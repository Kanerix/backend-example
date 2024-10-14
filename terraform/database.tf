data "azurerm_postgresql_flexible_server" "primary" {
  name                = "lerpz-infrastructure-pgs"
  resource_group_name = "lerpz-infrastructure-database"
}

resource "azurerm_postgresql_flexible_server_database" "app" {
  name      = "${local.repository_name}-${var.deploy_env}"
  server_id = data.azurerm_postgresql_flexible_server.primary.id
  collation = "en_US.utf8"
  charset   = "utf8"

  lifecycle {
    prevent_destroy = true
  }
}
