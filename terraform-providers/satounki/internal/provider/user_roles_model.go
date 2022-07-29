package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/resource"
)

func (d userRolesResourceData) PostBody() satounki.UserRolesPostBody {
	var roles []satounki.AccessRole
	for _, r := range d.AccessRoles {
		roles = append(roles, satounki.AccessRole(r.Value))
	}

	return satounki.UserRolesPostBody(roles)
}

func (d *userRolesResourceData) PostResponse(r satounki.UserRolesPostResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.String{Value: string(role)})
	}

	d.ID = types.String{Value: resource.UniqueId()}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.AccessRoles = roles
}

func (d userRolesResourceData) PutBody() satounki.UserRolesPutBody {
	var roles []satounki.AccessRole
	for _, r := range d.AccessRoles {
		roles = append(roles, satounki.AccessRole(r.Value))
	}

	return satounki.UserRolesPutBody(roles)
}

func (d *userRolesResourceData) PutResponse(r satounki.UserRolesPutResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.String{Value: string(role)})
	}

	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.AccessRoles = roles
}

func (d *userRolesResourceData) GetResponse(r satounki.UserRolesGetResponse) {
	var roles []types.String
	for _, role := range r {
		roles = append(roles, types.String{Value: string(role)})
	}

	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.AccessRoles = roles
}
