package provider

import (
	"context"
	"fmt"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
	"github.com/hashicorp/terraform-plugin-framework/types"
	"satounki"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ resource.Resource                = &userRolesResource{}
	_ resource.ResourceWithConfigure   = &userRolesResource{}
	_ resource.ResourceWithImportState = &userRolesResource{}
)

// NewUserRolesResource is a helper function to simplify the provider implementation.
func NewUserRolesResource() resource.Resource {
	return &userRolesResource{}
}

// userRolesResource is the resource implementation.
type userRolesResource struct {
	client *satounki.API
}

// Metadata returns the resource type name.
func (r *userRolesResource) Metadata(_ context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_user_roles"
}

func (r *userRolesResource) Schema(_ context.Context, _ resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: resourceDoc(userRolesResourceData{}),
		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "id"),
				Computed:    true,
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"last_updated": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "last_updated"),
				Computed:    true,
			},
			"email": schema.StringAttribute{
				Description: fieldDoc(userRolesResourceData{}, "email"),
				Required:    true,
			},
			"access_roles": schema.ListAttribute{
				ElementType: types.StringType,
				Description: fieldDoc(userRolesResourceData{}, "access_roles"),
				Required:    true,
			},
		},
	}
}

func (r *userRolesResource) Configure(_ context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
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

func (r *userRolesResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	// Retrieve values from plan
	var plan userRolesResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PostBody()

	response, _, err := r.client.UserRolesPost(plan.Email.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error creating user roles",
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

func (r *userRolesResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	// Get current state
	var state userRolesResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	response, _, err := r.client.UserRolesGet(state.Email.ValueString())
	if err != nil {
		resp.Diagnostics.AddError("Error reading user roles",
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

func (r *userRolesResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	// Retrieve values from plan
	var plan userRolesResourceData
	diags := req.Plan.Get(ctx, &plan)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	body := plan.PutBody()

	response, _, err := r.client.UserRolesPut(plan.Email.ValueString(), body)
	if err != nil {
		resp.Diagnostics.AddError("Error updating company",
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

func (r *userRolesResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	// Retrieve values from state
	var state userRolesResourceData
	diags := req.State.Get(ctx, &state)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	if _, _, err := r.client.UserRolesPost(
		state.Email.ValueString(),
		satounki.UserRolesPostBody([]satounki.AccessRole{})); err != nil {
		resp.Diagnostics.AddError("Client Error",
			err.Error(),
		)

		return
	}

	resp.State.RemoveResource(ctx)
}

func (r *userRolesResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	// Retrieve import ID and save to id attribute
	resource.ImportStatePassthroughID(ctx, path.Root("email"), req, resp)
}
