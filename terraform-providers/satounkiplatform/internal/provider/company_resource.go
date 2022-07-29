package provider

import (
	"context"

	"github.com/hashicorp/terraform-plugin-framework/diag"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/tfsdk"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure provider defined types fully satisfy framework interfaces
var _ tfsdk.ResourceType = companyResourceType{}
var _ tfsdk.Resource = companyResource{}
var _ tfsdk.ResourceWithImportState = companyResource{}

type companyResourceType struct{}

func (t companyResourceType) GetSchema(ctx context.Context) (tfsdk.Schema, diag.Diagnostics) {
	return tfsdk.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: resourceDoc(companyResourceData{}),

		Attributes: map[string]tfsdk.Attribute{
			"id": {
				Computed:            true,
				MarkdownDescription: fieldDoc(companyResourceData{}, "id"),
				PlanModifiers: tfsdk.AttributePlanModifiers{
					tfsdk.UseStateForUnknown(),
				},
				Type: types.StringType,
			},
			"last_updated": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "last_updated"),
				Type:                types.StringType,
				Computed:            true,
			},
			"name": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "name"),
				Type:                types.StringType,
				Required:            true,
			},
			"domain": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "domain"),
				Type:                types.StringType,
				Required:            true,
			},
			"root_user_email": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "root_user_email"),
				Type:                types.StringType,
				Required:            true,
			},
			"root_user_first_name": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "root_user_first_name"),
				Type:                types.StringType,
				Required:            true,
			},
			"root_user_last_name": {
				MarkdownDescription: fieldDoc(companyResourceData{}, "root_user_last_name"),
				Type:                types.StringType,
				Required:            true,
			},
		},
	}, nil
}

func (t companyResourceType) NewResource(ctx context.Context, in tfsdk.Provider) (tfsdk.Resource, diag.Diagnostics) {
	provider, diags := convertProviderType(in)

	return companyResource{
		provider: provider,
	}, diags
}

type companyResource struct {
	provider provider
}

func (r companyResource) Create(ctx context.Context, req tfsdk.CreateResourceRequest, resp *tfsdk.CreateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data companyResourceData

	diags := req.Config.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	body, err := data.PostBody()
	if err != nil {
		resp.Diagnostics.AddError("Error parsing expiry date",
			"The following format must be used: 2006-01-02 15:04:05",
		)

		return
	}

	response, _, err := r.provider.api.CompanyPost(body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating company",
			err.Error(),
		)

		return
	}

	data.PostResponse(response)

	diags = resp.State.Set(ctx, data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r companyResource) Read(ctx context.Context, req tfsdk.ReadResourceRequest, resp *tfsdk.ReadResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data companyResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.CompanyGet(data.ID.Value)
	if err != nil {
		resp.Diagnostics.AddError("Error reading company",
			err.Error(),
		)

		return
	}

	data.GetResponse(response)

	diags = resp.State.Set(ctx, data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r companyResource) Update(ctx context.Context, req tfsdk.UpdateResourceRequest, resp *tfsdk.UpdateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data companyResourceData

	diags := req.Plan.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	var state companyResourceData
	diags = req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	body, err := data.PutBody()
	if err != nil {
		resp.Diagnostics.AddError("Error parsing expiry date",
			"The following format must be used: 2006-01-02 15:04:05",
		)

		return
	}

	response, _, err := r.provider.api.CompanyPut(state.ID.Value, body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating company",
			err.Error(),
		)

		return
	}

	data.PutResponse(response)

	diags = resp.State.Set(ctx, data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r companyResource) Delete(ctx context.Context, req tfsdk.DeleteResourceRequest, resp *tfsdk.DeleteResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data companyResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	err := r.provider.api.CompanyDelete(data.ID.Value)
	if err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r companyResource) ImportState(ctx context.Context, req tfsdk.ImportResourceStateRequest, resp *tfsdk.ImportResourceStateResponse) {
	tfsdk.ResourceImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
