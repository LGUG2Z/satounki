package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
)

func (r *userAliasesResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(userAliasesResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "last_updated"),
				Computed:    true,
			},
			"email": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "email"),
				Required:    true,
			},
			"aws": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "aws"),
				Optional:    true,
			},
			"cloudflare": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "cloudflare"),
				Optional:    true,
			},
			"gcp": schema.StringAttribute{
				Description: fieldDoc(userAliasesResourceData{}, "gcp"),
				Optional:    true,
			},
		},
	}
}
