data "azurerm_subscription" "current" {
  subscription_id = "5509a305-b67f-4d6c-804e-b38fe72dc105"
}

data "github_repository" "current" {
  full_name = "lerpz-com/${local.repository_name}"
}

resource "azurerm_resource_group" "main" {
  name     = "${data.github_repository.current.name}-${var.deploy_env}"
  location = local.location
}
