data "azurerm_subscription" "current" {
  subscription_id = "5509a305-b67f-4d6c-804e-b38fe72dc105"
}

resource "azurerm_resource_group" "ext" {
  name     = "${local.repository_name}-ext"
  location = local.location
}
