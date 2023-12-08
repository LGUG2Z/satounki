package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (r *policyResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(policyResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(policyResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(policyResourceData{}, "last_updated"),
				Computed:    true,
			},
			"name": schema.StringAttribute{
				Description: fieldDoc(policyResourceData{}, "name"),
				Required:    true,
			},
			"description": schema.StringAttribute{
				Description: fieldDoc(policyResourceData{}, "description"),
				Required:    true,
			},
			"aws": schema.ListAttribute{
				ElementType: types.StringType,
				Description: fieldDoc(policyResourceData{}, "aws"),
				Optional:    true,
			},
			"cloudflare": schema.ListAttribute{
				ElementType: types.StringType,
				Description: fieldDoc(policyResourceData{}, "cloudflare"),
				Optional:    true,
			},
			"gcp": schema.ListAttribute{
				ElementType: types.StringType,
				Description: fieldDoc(policyResourceData{}, "gcp"),
				Optional:    true,
			},
		},
	}
}
