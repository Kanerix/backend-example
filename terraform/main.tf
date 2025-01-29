data "azurerm_client_config" "primary" {}

data "azurerm_subscription" "primary" {}

data "github_repository" "primary" {
  full_name = "lerpz-com/${local.repository_name}"
}

resource "azurerm_resource_group" "app" {
  name     = "${data.github_repository.primary.name}-${var.deploy_env}"
  location = local.location
}

resource "azurerm_user_assigned_identity" "app" {
  name                = "${local.repository_name}-mi"
  resource_group_name = azurerm_resource_group.app.name
  location            = azurerm_resource_group.app.location
}

resource "random_string" "pwd_secret" {
  length  = 32
  special = true
}
