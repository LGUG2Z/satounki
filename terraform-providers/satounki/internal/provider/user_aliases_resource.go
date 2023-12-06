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
	_ resource.Resource                = &userAliasesResource{}
	_ resource.ResourceWithConfigure   = &userAliasesResource{}
	_ resource.ResourceWithImportState = &userAliasesResource{}
)

// NewuserAliasesResource is a helper function to simplify the provider implementation.
func NewUserAliasesResource() resource.Resource {
	return &userAliasesResource{}
}

// userAliasesResource is the resource implementation.
type userAliasesResource struct {
	client *satounki.API
}

// Metadata returns the resource type name.
func (r *userAliasesResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_user_aliases"
}

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

func (r *userAliasesResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
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

func (r *userAliasesResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	// Retrieve values from plan
	var plan userAliasesResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PostBody()

	response, _, err := r.client.UserAliasesPost(plan.Email.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating user aliases",
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

func (r *userAliasesResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	// Get current state
	var state userAliasesResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.client.UserAliasesGet(state.Email.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Error reading user aliases",
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

func (r *userAliasesResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	// Retrieve values from plan
	var plan userAliasesResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PutBody()

	response, _, err := r.client.UserAliasesPut(plan.Email.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating user aliases",
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

func (r *userAliasesResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	// Retrieve values from state
	var state userAliasesResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	err := r.client.UserAliasesDelete(state.Email.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r *userAliasesResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	// Retrieve import ID and save to id attribute
	resource.ImportStatePassthroughID(ctx, path.Root("email"), req, resp)
}
