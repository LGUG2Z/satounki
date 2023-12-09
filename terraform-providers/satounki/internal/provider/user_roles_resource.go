package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (r *userRolesResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(userRolesResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "last_updated"),
				Computed:    true,
			},
			"email": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "email"),
				Required:    true,
			},
			"access_roles": schema.ListAttribute{
				ElementType: types.StringType,
				Description: fieldDoc(userRolesResourceData{}, "access_roles"),
				Required:    true,
			},
		},
	}
}
