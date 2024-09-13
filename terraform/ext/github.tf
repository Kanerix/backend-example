resource "github_repository" "repo" {
  name        = local.repository_name
  description = "An API created using Axum"
  visibility  = "public"

  vulnerability_alerts = true

  has_projects  = true
  has_downloads = true
  has_issues    = true
}

resource "github_repository_environment" "prod" {
  environment         = "prod"
  repository          = github_repository.repo.name
  prevent_self_review = true

  deployment_branch_policy {
    protected_branches     = true
    custom_branch_policies = false
  }
}

resource "github_repository_environment" "stag" {
  environment         = "stag"
  repository          = github_repository.repo.name
  prevent_self_review = true

  deployment_branch_policy {
    protected_branches     = true
    custom_branch_policies = false
  }
}

resource "github_actions_variable" "project_type" {
  repository    = local.repository_name
  variable_name = "PROJECT_TYPE"
  value         = "terraform"
}

resource "github_actions_variable" "deploy_platform" {
  repository    = local.repository_name
  variable_name = "DEPLOY_PLATFORM"
  value         = "azure"
}

resource "github_actions_variable" "azure_client_id" {
  repository    = local.repository_name
  variable_name = "AZURE_CLIENT_ID"
  value         = azurerm_user_assigned_identity.deployment.client_id
}

resource "github_actions_variable" "azure_subscription_id" {
  repository    = local.repository_name
  variable_name = "AZURE_SUBSCRIPTION_ID"
  value         = data.azurerm_subscription.current.subscription_id
}
