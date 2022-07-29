package provider

import (
	"context"

	"github.com/hashicorp/terraform-plugin-framework/diag"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/tfsdk"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure provider defined types fully satisfy framework interfaces
var _ tfsdk.ResourceType = userAliasesResourceType{}
var _ tfsdk.Resource = userAliasesResource{}
var _ tfsdk.ResourceWithImportState = userAliasesResource{}

type userAliasesResourceType struct{}

func (t userAliasesResourceType) GetSchema(ctx context.Context) (tfsdk.Schema, diag.Diagnostics) {
	return tfsdk.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: resourceDoc(userAliasesResourceData{}),

		Attributes: map[string]tfsdk.Attribute{
			"id": {
				Computed:            true,
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "id"),
				PlanModifiers: tfsdk.AttributePlanModifiers{
					tfsdk.UseStateForUnknown(),
				},
				Type: types.StringType,
			},
			"last_updated": {
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "last_updated"),
				Type:                types.StringType,
				Computed:            true,
			},
			"email": {
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "email"),
				Type:                types.StringType,
				Required:            true,
			},
			"aws": {
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "aws"),
				Type:                types.StringType,
				Optional:            true,
			},
			"cloudflare": {
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "cloudflare"),
				Type:                types.StringType,
				Optional:            true,
			},
			"gcp": {
				MarkdownDescription: fieldDoc(userAliasesResourceData{}, "gcp"),
				Type:                types.StringType,
				Optional:            true,
			},
		},
	}, nil
}

func (t userAliasesResourceType) NewResource(ctx context.Context, in tfsdk.Provider) (tfsdk.Resource, diag.Diagnostics) {
	provider, diags := convertProviderType(in)

	return userAliasesResource{
		provider: provider,
	}, diags
}

type userAliasesResource struct {
	provider provider
}

func (r userAliasesResource) Create(ctx context.Context, req tfsdk.CreateResourceRequest, resp *tfsdk.CreateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data userAliasesResourceData

	diags := req.Config.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.UserAliasesPost(data.Email.Value, data.PostBody())
	if err != nil {
		resp.Diagnostics.AddError("Error creating user aliases",
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

func (r userAliasesResource) Read(ctx context.Context, req tfsdk.ReadResourceRequest, resp *tfsdk.ReadResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data userAliasesResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.UserAliasesGet(data.Email.Value)
	if err != nil {
		resp.Diagnostics.AddError("Error reading user aliases",
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

func (r userAliasesResource) Update(ctx context.Context, req tfsdk.UpdateResourceRequest, resp *tfsdk.UpdateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data userAliasesResourceData

	diags := req.Plan.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	var state userAliasesResourceData
	diags = req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.UserAliasesPut(state.Email.Value, data.PutBody())
	if err != nil {
		resp.Diagnostics.AddError("Error updating user aliases",
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

func (r userAliasesResource) Delete(ctx context.Context, req tfsdk.DeleteResourceRequest, resp *tfsdk.DeleteResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data userAliasesResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	if err := r.provider.api.UserAliasesDelete(data.Email.Value); err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r userAliasesResource) ImportState(ctx context.Context, req tfsdk.ImportResourceStateRequest, resp *tfsdk.ImportResourceStateResponse) {
	tfsdk.ResourceImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
