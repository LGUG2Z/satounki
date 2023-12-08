set dotenv-load := true

secrets:
    sops --decrypt secrets.yaml | yq .development.env > .env
    sops --decrypt secrets.yaml | yq .development.tunnel_credentials_json > tunnel-credentials.json
    sops --decrypt secrets.yaml | yq .development.tunnel_config_yaml > tunnel-config.yaml

fmt:
    gofmt -w ./satounki-go
    gofmt -w ./satounki-platform-go
    gofmt -w ./terraform-providers/satounkiplatform
    gofmt -w ./terraform-providers/satounki
    cd terraform-providers/satounkiplatform && golangci-lint run
    cd terraform-providers/satounki && golangci-lint run
    prettier -w ./satounki-ts
    prettier -w README.md
    tsc --noEmit -p ./satounki-ts/tsconfig.json
    curlylint api/templates
    djhtml api/templates
    terraform fmt -recursive terraform
    cargo fmt
    cargo clippy

bacon:
    bacon -j clippy -w

gen-clean:
    rm -rf json-v1 json-platform
    mkdir -p json-v1 json-platform

gen-common:
    cargo run --package common-gen

gen-common-patches:
    quicktype -l go -s schema ./json-v1/*.json --no-multi-file-output -o "./satounki-go/types.generated.go" --package satounki
    quicktype -l go -s schema ./json-platform/*.json --no-multi-file-output -o "./satounki-platform-go/types.generated.go" --package satounki
    quicktype -l typescript -s schema ./json-v1/*.json -o "./satounki-ts/types.generated.ts" --just-types --no-combine-classes
    echo "" >>./satounki-ts/types.generated.ts
    echo "export type UserRolesPutRequest = UserRolesPutResponse;" >>./satounki-ts/types.generated.ts
    echo "export type UserRolesPostRequest = UserRolesPutResponse;" >>./satounki-ts/types.generated.ts
    echo "export type UserRolesGetResponse = UserRolesPutResponse;" >>./satounki-ts/types.generated.ts
    echo "export type UserRolesPostResponse = UserRolesPutResponse;" >>./satounki-ts/types.generated.ts
    sd "RequestAliasPatchRequest" "RequestAliasPatchRequestEnum" ./satounki-ts/api.generated.ts

gen:
    just gen-clean
    just gen-common
    just gen-common-patches
    just fmt

rebuild-provider-satounki:
    cd terraform-providers/satounki && go mod tidy && go mod vendor && go build -o ~/.terraform.d/plugins/registry.terraform.io/hashicorp/satounki/0.1/linux_amd64/terraform-provider-satounki_v0.1
    cd terraform-providers/satounki && go generate

rebuild-provider-satounkiplatform:
    cd terraform-providers/satounkiplatform && go mod tidy && go mod vendor && go build -o ~/.terraform.d/plugins/registry.terraform.io/hashicorp/satounkiplatform/0.1/linux_amd64/terraform-provider-satounkiplatform_v0.1
    cd terraform-providers/satounkiplatform && go generate

rebuild-providers:
    just rebuild-provider-satounki
    just rebuild-provider-satounkiplatform

init-terraform:
    cd terraform && rm -f .terraform.lock.hcl && rm -f *tfstate* && terraform init

tf:
    just gen
    just rebuild-providers
    just init-terraform

rolescraper:
    cargo run --package rolescraper

api:
    cargo run --package api

worker:
    cargo run --package client -- --config client/src/configuration.yaml

tunnel:
    cloudflared tunnel --config ./tunnel-config.yaml run satounki-dev

start:
    process-compose

set positional-arguments
satounki *args:
    #!/usr/bin/env bash
    export SATOUNKI_USER_TOKEN="$1"
    shift
    cargo run -p satounki -- "$@"
