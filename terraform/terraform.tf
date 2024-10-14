terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.115"
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
