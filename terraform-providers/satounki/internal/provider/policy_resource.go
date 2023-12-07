package provider

import (
	"context"
	"fmt"
	"satounki"

	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"

	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ resource.Resource                = &policyResource{}
	_ resource.ResourceWithConfigure   = &policyResource{}
	_ resource.ResourceWithImportState = &policyResource{}
)

// NewpolicyResource is a helper function to simplify the provider implementation.
func NewPolicyResource() resource.Resource {
	return &policyResource{}
}

// policyResource is the resource implementation.
type policyResource struct {
	client *satounki.API
}

// Metadata returns the resource type name.
func (r *policyResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_policy"
}

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

func (r *policyResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
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

func (r *policyResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	// Retrieve values from plan
	var plan policyResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PostRequest()

	response, _, err := r.client.PolicyPost(body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating policy",
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

func (r *policyResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	// Get current state
	var state policyResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.client.PolicyGet(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Error reading policy",
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

func (r *policyResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	// Retrieve values from plan
	var plan policyResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PutRequest()

	response, _, err := r.client.PolicyPut(plan.ID.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating policy",
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

func (r *policyResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	// Retrieve values from state
	var state policyResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	err := r.client.PolicyDelete(state.ID.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r *policyResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	// Retrieve import ID and save to id attribute
	resource.ImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}
