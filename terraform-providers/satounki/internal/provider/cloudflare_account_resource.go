package provider

import (
	"context"
	"fmt"
	"satounki"

	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ resource.Resource                = &cloudflareAccountResource{}
	_ resource.ResourceWithConfigure   = &cloudflareAccountResource{}
	_ resource.ResourceWithImportState = &cloudflareAccountResource{}
)

// NewCompanyResource is a helper function to simplify the provider implementation.
func NewCloudflareAccountResource() resource.Resource {
	return &cloudflareAccountResource{}
}

// cloudflareAccountResource is the resource implementation.
type cloudflareAccountResource struct {
	client *satounki.API
}

func (r *cloudflareAccountResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_cloudflare_account"
}

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

func (r *cloudflareAccountResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
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

func (r *cloudflareAccountResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	// Retrieve values from plan
	var plan cloudflareAccountResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PostRequest()

	response, _, err := r.client.SettingsCloudflareAccountPost(body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating CLOUDFLARE account",
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

func (r *cloudflareAccountResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	// Get current state
	var state cloudflareAccountResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.client.SettingsCloudflareAccountGet(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Error reading CLOUDFLARE account",
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

func (r *cloudflareAccountResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	// Retrieve values from plan
	var plan cloudflareAccountResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PutRequest()

	response, _, err := r.client.SettingsCloudflareAccountPut(plan.ID.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating CLOUDFLARE account",
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

func (r *cloudflareAccountResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	// Retrieve values from state
	var state cloudflareAccountResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	err := r.client.SettingsCloudflareAccountDelete(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r *cloudflareAccountResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	// Retrieve import ID and save to id attribute
	resource.ImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
