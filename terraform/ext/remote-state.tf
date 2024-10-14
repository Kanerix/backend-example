resource "random_string" "resource_code" {
  length  = 5
  special = false
  upper   = false
}

resource "azurerm_storage_account" "tfstate" {
  name                            = "tfstate${random_string.resource_code.result}"
  resource_group_name             = azurerm_resource_group.ext.name
  location                        = azurerm_resource_group.ext.location
  account_tier                    = "Standard"
  account_replication_type        = "LRS"
  allow_nested_items_to_be_public = false
}

resource "azurerm_storage_container" "tfstate_ext" {
  name                  = "tfstate-ext"
  storage_account_name  = azurerm_storage_account.tfstate.name
  container_access_type = "private"
}

resource "azurerm_storage_container" "tfstate_stag" {
  name                  = "tfstate-stag"
  storage_account_name  = azurerm_storage_account.tfstate.name
  container_access_type = "private"
}

resource "azurerm_storage_container" "tfstate_prod" {
  name                  = "tfstate-prod"
  storage_account_name  = azurerm_storage_account.tfstate.name
  container_access_type = "private"
}
