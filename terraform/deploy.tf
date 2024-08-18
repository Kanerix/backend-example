resource "azurerm_resource_group" "app" {
  name     = "${local.repository_name}-${var.env}"
  location = local.location
}

resource "azurerm_resource_group" "deployment" {
  name     = "${local.repository_name}-deployment"
  location = local.location
}

resource "azurerm_user_assigned_identity" "deployment-mi" {
  name                = "${local.repository_name}-deployment-mi"
  resource_group_name = azurerm_resource_group.deployment.name
  location            = azurerm_resource_group.deployment.location
}

resource "azurerm_federated_identity_credential" "env_stag" {
  name                = "gh-actions-env-stag"
  resource_group_name = azurerm_resource_group.deployment.name
  parent_id           = azurerm_user_assigned_identity.deployment-mi.id
  audience            = ["api://AzureADTokenExchange"]
  issuer              = "https://token.actions.githubusercontent.com"
  subject             = "repo:lerpz-com/${local.repository_name}:environment:refs/heads/staging"
}

resource "azurerm_federated_identity_credential" "env_prod" {
  name                = "gh-actions-env-prod"
  resource_group_name = azurerm_resource_group.deployment.name
  parent_id           = azurerm_user_assigned_identity.deployment-mi.id
  audience            = ["api://AzureADTokenExchange"]
  issuer              = "https://token.actions.githubusercontent.com"
  subject             = "repo:lerpz-com/${local.repository_name}:environment:refs/heads/main"
}
