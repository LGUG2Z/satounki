package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
)

// Schema defines the schema for the resource.
func (r *companyResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(companyResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "last_updated"),
				Computed:    true,
			},
			"name": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "name"),
				Required:    true,
			},
			"domain": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "domain"),
				Required:    true,
			},
			"root_user_email": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "root_user_email"),
				Required:    true,
			},
			"root_user_first_name": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "root_user_first_name"),
				Required:    true,
			},
			"root_user_last_name": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "root_user_last_name"),
				Required:    true,
			},
			"api_token": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "api_token"),
				Computed:    true,
				Sensitive:   true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"worker_key": schema.StringAttribute{
				Description: fieldDoc(companyResourceData{}, "worker_key"),
				Computed:    true,
				Sensitive:   true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
		},
	}
}
