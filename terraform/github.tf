resource "github_repository" "repo" {
  name          = local.repository_name
  visibility    = "public"
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

resource "github_actions_variable" "platform" {
  repository    = local.repository_name
  variable_name = "DEPLOYMENT_PLATFORM"
  value         = "azure"
}
