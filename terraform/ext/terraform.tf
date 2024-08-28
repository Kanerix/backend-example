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
    resource_group_name  = "lerpz-backend-tfstate"
    storage_account_name = "tfstatev0wol"
    container_name       = "tfstate-deploy"
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
  github_orginization = "lerpz-com"
  location            = "West Europe"
  repository_name     = "lerpz-backend"
}
