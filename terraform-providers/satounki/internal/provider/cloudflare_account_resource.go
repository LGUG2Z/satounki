package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
)

func (r *cloudflareAccountResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(cloudflareAccountResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(cloudflareAccountResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(cloudflareAccountResourceData{}, "last_updated"),
				Computed:    true,
			},
			"account": schema.StringAttribute{
				Description: fieldDoc(cloudflareAccountResourceData{}, "account"),
				Required:    true,
			},
			"approvals_required": schema.Int64Attribute{
				Description: fieldDoc(cloudflareAccountResourceData{}, "approvals_required"),
				Required:    true,
			},
			"admin_approval_required": schema.BoolAttribute{
				Description: fieldDoc(cloudflareAccountResourceData{}, "admin_approval_required"),
				Required:    true,
			},
		},
	}
}
