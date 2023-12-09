terraform {
  required_providers {
    satounkiplatform = {
      source  = "registry.terraform.io/hashicorp/satounkiplatform"
      version = "0.1"
    }
    satounki = {
      source  = "registry.terraform.io/hashicorp/satounki"
      version = "0.1"
    }
  }
}

provider "satounkiplatform" {
  base_url  = "http://localhost:8080/platform"
  api_token = "e-mo-tion"
}

provider "satounki" {
  base_url  = "http://localhost:8080/v1"
  api_token = satounkiplatform_company.satounki.api_token
}

resource "satounkiplatform_company" "satounki" {
  name                 = "Satounki Development"
  domain               = "satounki.com"
  root_user_email      = "lgug2z@satounki.com"
  root_user_first_name = "Jeezy"
  root_user_last_name  = "LGUG2Z"
}

resource "satounki_aws_account" "all" {
  for_each                = toset(["x-development", "x-production"])
  account                 = each.value
  approvals_required      = 1
  admin_approval_required = replace(each.value, "production", "") != each.value
}

resource "satounki_gcp_project" "all" {
  for_each                = toset(["x-development", "x-production"])
  project                 = each.value
  approvals_required      = 1
  admin_approval_required = replace(each.value, "production", "") != each.value
}

resource "satounki_cloudflare_account" "company" {
  account                 = "x"
  approvals_required      = 1
  admin_approval_required = false
}

resource "satounki_policy" "storage_analytics_ro" {
  name        = "storage_analytics_ro"
  description = "Read only access to Storage on GCP and AWS, and Analytics on Cloudflare"
  gcp = [
    "roles/storage.objectViewer",
  ]
  aws = [
    "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
  ]
  cloudflare = [
    "Analytics"
  ]
}

locals {
  users_roles = {
    "lgug2z@satounki.com" = [
      "administrator",
      "approver",
      "user",
    ]
  }

  user = [
    "lgug2z@satounki.com",
  ]

  approver = [
    "lgug2z@satounki.com"
  ]

  administrator = [
    "lgug2z@satounki.com"
  ]

  users_roles_map = {
    for i, v in distinct(concat(local.user, local.administrator, local.approver)) : v => sort(compact([
      contains(local.administrator, v) ? "administrator" : "",
      contains(local.approver, v) ? "approver" : "",
      contains(local.user, v) ? "user" : "",
    ]))
  }
}

resource "satounki_user_roles" "roles" {
  for_each = local.users_roles_map
  email    = each.key

  access_roles = each.value
}

resource "satounki_user_aliases" "aliases" {
  email      = "lgug2z@satounki.com"
  aws        = "Jeezy"
  cloudflare = "jz"
  #    gcp = "lgug2z@satounki.com"
}
