package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
)

func (r *gcpProjectResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(gcpProjectResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(gcpProjectResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(gcpProjectResourceData{}, "last_updated"),
				Computed:    true,
			},
			"project": schema.StringAttribute{
				Description: fieldDoc(gcpProjectResourceData{}, "project"),
				Required:    true,
			},
			"approvals_required": schema.Int64Attribute{
				Description: fieldDoc(gcpProjectResourceData{}, "approvals_required"),
				Required:    true,
			},
			"admin_approval_required": schema.BoolAttribute{
				Description: fieldDoc(gcpProjectResourceData{}, "admin_approval_required"),
				Required:    true,
			},
		},
	}
}
