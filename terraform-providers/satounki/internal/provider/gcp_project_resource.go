package provider

import (
	"context"

	"github.com/hashicorp/terraform-plugin-framework/diag"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/tfsdk"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure provider defined types fully satisfy framework interfaces
var _ tfsdk.ResourceType = gcpProjectResourceType{}
var _ tfsdk.Resource = gcpProjectResource{}
var _ tfsdk.ResourceWithImportState = gcpProjectResource{}

type gcpProjectResourceType struct{}

func (t gcpProjectResourceType) GetSchema(ctx context.Context) (tfsdk.Schema, diag.Diagnostics) {
	return tfsdk.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: resourceDoc(gcpProjectResourceData{}),

		Attributes: map[string]tfsdk.Attribute{
			"id": {
				Computed:            true,
				MarkdownDescription: fieldDoc(gcpProjectResourceData{}, "id"),
				PlanModifiers: tfsdk.AttributePlanModifiers{
					tfsdk.UseStateForUnknown(),
				},
				Type: types.StringType,
			},
			"last_updated": {
				MarkdownDescription: fieldDoc(gcpProjectResourceData{}, "last_updated"),
				Type:                types.StringType,
				Computed:            true,
			},
			"project": {
				MarkdownDescription: fieldDoc(gcpProjectResourceData{}, "project"),
				Type:                types.StringType,
				Required:            true,
			},
			"approvals_required": {
				MarkdownDescription: fieldDoc(gcpProjectResourceData{}, "approvals_required"),
				Type:                types.Int64Type,
				Required:            true,
			},
			"admin_approval_required": {
				MarkdownDescription: fieldDoc(gcpProjectResourceData{}, "admin_approval_required"),
				Type:                types.BoolType,
				Required:            true,
			},
		},
	}, nil
}

func (t gcpProjectResourceType) NewResource(ctx context.Context, in tfsdk.Provider) (tfsdk.Resource, diag.Diagnostics) {
	provider, diags := convertProviderType(in)

	return gcpProjectResource{
		provider: provider,
	}, diags
}

type gcpProjectResource struct {
	provider provider
}

func (r gcpProjectResource) Create(ctx context.Context, req tfsdk.CreateResourceRequest, resp *tfsdk.CreateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data gcpProjectResourceData

	diags := req.Config.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.SettingsGcpProjectPost(data.PostBody())
	if err != nil {
		resp.Diagnostics.AddError("Error creating GCP project",
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

func (r gcpProjectResource) Read(ctx context.Context, req tfsdk.ReadResourceRequest, resp *tfsdk.ReadResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data gcpProjectResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.SettingsGcpProjectGet(data.ID.Value)
	if err != nil {
		resp.Diagnostics.AddError("Error reading GCP project",
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

func (r gcpProjectResource) Update(ctx context.Context, req tfsdk.UpdateResourceRequest, resp *tfsdk.UpdateResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data gcpProjectResourceData

	diags := req.Plan.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	var state gcpProjectResourceData
	diags = req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.provider.api.SettingsGcpProjectPut(state.ID.Value, data.PutBody())
	if err != nil {
		resp.Diagnostics.AddError("Error updating GCP project",
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

func (r gcpProjectResource) Delete(ctx context.Context, req tfsdk.DeleteResourceRequest, resp *tfsdk.DeleteResourceResponse) {
	if !r.provider.configured {
		resp.Diagnostics.AddError(
			"Provider not configured",
			"The provider hasn't been configured before apply, likely because it depends on an unknown value from another resource. This leads to weird stuff happening, so we'd prefer if you didn't do that. Thanks!",
		)
		return
	}

	var data gcpProjectResourceData

	diags := req.State.Get(ctx, &data)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	if err := r.provider.api.SettingsGcpProjectDelete(data.ID.Value); err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r gcpProjectResource) ImportState(ctx context.Context, req tfsdk.ImportResourceStateRequest, resp *tfsdk.ImportResourceStateResponse) {
	tfsdk.ResourceImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
