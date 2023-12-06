package provider

import (
	"context"
	"fmt"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
	"satounki"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ resource.Resource                = &gcpProjectResource{}
	_ resource.ResourceWithConfigure   = &gcpProjectResource{}
	_ resource.ResourceWithImportState = &gcpProjectResource{}
)

// NewCompanyResource is a helper function to simplify the provider implementation.
func NewGcpProjectResource() resource.Resource {
	return &gcpProjectResource{}
}

// gcpProjectResource is the resource implementation.
type gcpProjectResource struct {
	client *satounki.API
}

func (r *gcpProjectResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_gcp_project"
}

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

func (r *gcpProjectResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
	if req.ProviderData == nil {
		return
	}

	client, ok := req.ProviderData.(satounki.API)

	if !ok {
		resp.Diagnostics.AddError(
			"Unexpected Data Source Configure Type",
			fmt.Sprintf("Expected *satounki.API, got: %T. Please report this issue to the provider developers.", req.ProviderData),
		)

		return
	}

	r.client = &client
}

func (r *gcpProjectResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	// Retrieve values from plan
	var plan gcpProjectResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PostBody()

	response, _, err := r.client.SettingsGcpProjectPost(body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating GCP project",
			err.Error(),
		)

		return
	}

	plan.PostResponse(response)

	// Set state to fully populated data
	diags = resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r *gcpProjectResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	// Get current state
	var state gcpProjectResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.client.SettingsGcpProjectGet(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Error reading GCP project",
			err.Error(),
		)

		return
	}

	state.GetResponse(response)

	// Set refreshed state
	diags = resp.State.Set(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r *gcpProjectResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	// Retrieve values from plan
	var plan gcpProjectResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PutBody()

	response, _, err := r.client.SettingsGcpProjectPut(plan.ID.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating GCP project",
			err.Error(),
		)

		return
	}

	plan.PutResponse(response)

	diags = resp.State.Set(ctx, plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}
}

func (r *gcpProjectResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	// Retrieve values from state
	var state gcpProjectResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	err := r.client.SettingsGcpProjectDelete(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r *gcpProjectResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	// Retrieve import ID and save to id attribute
	resource.ImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
