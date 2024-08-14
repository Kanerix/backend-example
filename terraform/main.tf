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
    storage_account_name = "tfstateevt3p"
    container_name       = "tfstate"
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
  location        = "West Europe"
  repository_name = "lerpz-backend"
}

variable "ENV" {
  description = "Deployment environment"
  type        = string
  default     = "staging"
}
