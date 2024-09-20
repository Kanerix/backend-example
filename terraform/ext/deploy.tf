resource "azurerm_user_assigned_identity" "deploy" {
  name                = "${local.repository_name}-deploy-mi"
  resource_group_name = azurerm_resource_group.ext.name
  location            = azurerm_resource_group.ext.location
}

resource "azurerm_role_assignment" "sub_owner" {
  scope                = data.azurerm_subscription.current.id
  role_definition_name = "Owner"
  principal_id         = azurerm_user_assigned_identity.deploy.principal_id
}

resource "azurerm_federated_identity_credential" "env_stag" {
  name                = "gh-actions-env-stag"
  resource_group_name = azurerm_resource_group.ext.name
  parent_id           = azurerm_user_assigned_identity.deploy.id
  audience            = ["api://AzureADTokenExchange"]
  issuer              = "https://token.actions.githubusercontent.com"
  subject             = "repo:${local.github_orginization}/${local.repository_name}:environment:${github_repository_environment.stag.environment}"
}

resource "azurerm_federated_identity_credential" "env_prod" {
  name                = "gh-actions-env-prod"
  resource_group_name = azurerm_resource_group.ext.name
  parent_id           = azurerm_user_assigned_identity.deploy.id
  audience            = ["api://AzureADTokenExchange"]
  issuer              = "https://token.actions.githubusercontent.com"
  subject             = "repo:${local.github_orginization}/${local.repository_name}:environment:${github_repository_environment.prod.environment}"
}
