package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/id"
)

func (d userRolesResourceData) PostRequest() satounki.UserRolesPostRequest {
	var roles []satounki.AccessRole
	for _, r := range d.AccessRoles {
		roles = append(roles, satounki.AccessRole(r.ValueString()))
	}

	return satounki.UserRolesPostRequest(roles)
}

func (d *userRolesResourceData) PostResponse(r satounki.UserRolesPostResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.StringValue(string(role)))
	}

	d.ID = types.StringValue(id.UniqueId())
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.AccessRoles = roles
}

func (d userRolesResourceData) PutRequest() satounki.UserRolesPutRequest {
	var roles []satounki.AccessRole
	for _, r := range d.AccessRoles {
		roles = append(roles, satounki.AccessRole(r.ValueString()))
	}

	return satounki.UserRolesPutRequest(roles)
}

func (d *userRolesResourceData) PutResponse(r satounki.UserRolesPutResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.StringValue(string(role)))
	}

	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.AccessRoles = roles
}

func (d *userRolesResourceData) GetResponse(r satounki.UserRolesGetResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.StringValue(string(role)))
	}

	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.AccessRoles = roles
}
