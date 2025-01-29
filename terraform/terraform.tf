terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }

  backend "azurerm" {
    resource_group_name  = "lerpz-backend-ext"
    storage_account_name = "tfstatekvbja"
    key                  = "terraform.tfstate"
  }

  required_version = ">= 1.9.2"
}

provider "azurerm" {
  subscription_id = "5509a305-b67f-4d6c-804e-b38fe72dc105"
  features {}
}

provider "github" {
  owner = "lerpz-com"
}

locals {
  location            = "West Europe"
  github_orginization = "lerpz-com"
  repository_name     = "lerpz-backend"
}
