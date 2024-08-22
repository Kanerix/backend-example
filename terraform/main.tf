data "azurerm_subscription" "primary" {
  subscription_id = "5509a305-b67f-4d6c-804e-b38fe72dc105"
}

data "github_repository" "primary" {
  full_name = "lerpz-com/${local.repository_name}"
}

resource "azurerm_resource_group" "app" {
  name     = "${data.github_repository.primary.name}-${var.deploy_env}"
  location = local.location
}
