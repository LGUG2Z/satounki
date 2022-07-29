package provider

import (
	"context"

	"github.com/hashicorp/terraform-plugin-framework/diag"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/tfsdk"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure provider defined types fully satisfy framework interfaces
var _ tfsdk.ResourceType = policyResourceType{}
var _ tfsdk.Resource = policyResource{}
var _ tfsdk.ResourceWithImportState = policyResource{}

type policyResourceType struct{}

func (t policyResourceType) GetSchema(ctx context.Context) (tfsdk.Schema, diag.Diagnostics) {
	return tfsdk.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: resourceDoc(policyResourceData{}),

		Attributes: map[string]tfsdk.Attribute{
			"id": {
				Computed:            true,
				MarkdownDescription: fieldDoc(policyResourceData{}, "id"),
				PlanModifiers: tfsdk.AttributePlanModifiers{
					tfsdk.UseStateForUnknown(),
				},
				Type: types.StringType,
			},
			"last_updated": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "last_updated"),
				Type:                types.StringType,
				Computed:            true,
			},
			"name": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "name"),
				Type:                types.StringType,
				Required:            true,
			},
			"description": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "description"),
				Type:                types.StringType,
				Required:            true,
			},
			"aws": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "aws"),
				Type: types.ListType{
					ElemType: types.StringType,
				},
				Optional: true,
			},
			"cloudflare": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "cloudflare"),
				Type: types.ListType{
					ElemType: types.StringType,
				},
				Optional: true,
			},
			"gcp": {
				MarkdownDescription: fieldDoc(policyResourceData{}, "gcp"),
				Type: types.ListType{
					ElemType: types.StringType,
				},
				Optional: true,
			},
		},
	}, nil
}

func (t policyResourceType) NewResource(ctx context.Context, in tfsdk.Provider) (tfsdk.Resource, diag.Diagnostics) {
	provider, diags := convertProviderType(in)

	return policyResource{
		provider: provider,
	}, diags
}

type policyResource struct {
	provider provider
}

func (r policyResource) Create(ctx context.Context, req tfsdk.CreateResourceRequest, resp *tfsdk.CreateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data policyResourceData

	diags := req.Config.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.PolicyPost(data.PostBody())
	if err != nil {
		resp.Diagnostics.AddError("Error creating policy",
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

func (r policyResource) Read(ctx context.Context, req tfsdk.ReadResourceRequest, resp *tfsdk.ReadResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data policyResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.PolicyGet(data.ID.Value)
	if err != nil {
		resp.Diagnostics.AddError("Error reading policy",
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

func (r policyResource) Update(ctx context.Context, req tfsdk.UpdateResourceRequest, resp *tfsdk.UpdateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data policyResourceData

	diags := req.Plan.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	var state policyResourceData
	diags = req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.PolicyPut(state.ID.Value, data.PutBody())
	if err != nil {
		resp.Diagnostics.AddError("Error updating policy",
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

func (r policyResource) Delete(ctx context.Context, req tfsdk.DeleteResourceRequest, resp *tfsdk.DeleteResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data policyResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	if err := r.provider.api.PolicyDelete(data.ID.Value); err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r policyResource) ImportState(ctx context.Context, req tfsdk.ImportResourceStateRequest, resp *tfsdk.ImportResourceStateResponse) {
	tfsdk.ResourceImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
