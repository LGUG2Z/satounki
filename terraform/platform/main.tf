terraform {
  required_providers {
    satounkiplatform = {
      source  = "registry.terraform.io/hashicorp/satounkiplatform"
      version = "0.1"
    }
  }
}

provider "satounkiplatform" {
  base_url  = "http://localhost:8080/platform"
  api_token = "e-mo-tion"
}

resource "satounkiplatform_company" "satounki" {
  name                 = "Satounki Development"
  domain               = "satounki.com"
  root_user_email      = "lgug2z@satounki.com"
  root_user_first_name = "Jeezy"
  root_user_last_name  = "LGUG2Z"
}

