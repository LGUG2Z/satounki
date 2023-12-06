package provider

import (
	"context"
	"fmt"
	"github.com/hashicorp/go-cleanhttp"
	"os"
	"satounki"

	"github.com/hashicorp/terraform-plugin-framework/datasource"
	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/provider"
	"github.com/hashicorp/terraform-plugin-framework/provider/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-log/tflog"
)

// Ensure the implementation satisfies the expected interfaces.
var (
	_ provider.Provider = &satounkiProvider{}
)

// New is a helper function to simplify provider server and testing implementation.
func New(version string) func() provider.Provider {
	return func() provider.Provider {
		return &satounkiProvider{
			version: version,
		}
	}
}

// satounkiProvider is the provider implementation.
type satounkiProvider struct {
	// version is set to the provider version on release, "dev" when the
	// provider is built and ran locally, and "test" when running acceptance
	// testing.
	version string
}

// satounkiProviderModel maps provider schema data to a Go type.
type satounkiProviderModel struct {
	APIToken types.String `tfsdk:"api_token"`
	BaseURL  types.String `tfsdk:"base_url"`
}

// Metadata returns the provider type name.
func (p *satounkiProvider) Metadata(_ context.Context, _ provider.MetadataRequest, resp *provider.MetadataResponse) {
	resp.TypeName = "satounki"
	resp.Version = p.version
}

// Schema defines the provider-level schema for configuration data.
func (p *satounkiProvider) Schema(_ context.Context, _ provider.SchemaRequest, resp *provider.SchemaResponse) {
	resp.Schema = schema.Schema{
		Description: "Interact with Satounki",
		Attributes: map[string]schema.Attribute{
			"api_token": schema.StringAttribute{
				Description: "API Token",
				Required:    true,
				Sensitive:   true,
			},
			"base_url": schema.StringAttribute{
				Description: "Satounki API Base URL",
				Required:    true,
			},
		},
	}
}

func (p *satounkiProvider) Configure(ctx context.Context, req provider.ConfigureRequest, resp *provider.ConfigureResponse) {
	tflog.Info(ctx, "Configuring HashiCups client")

	// Retrieve provider data from configuration
	var config satounkiProviderModel
	diags := req.Config.Get(ctx, &config)
	resp.Diagnostics.Append(diags...)
	if resp.Diagnostics.HasError() {
		return
	}

	// If practitioner provided a configuration value for any of the
	// attributes, it must be a known value.

	if config.BaseURL.IsUnknown() {
		resp.Diagnostics.AddAttributeError(
			path.Root("baseURL"),
			"Unknown Satounki Base URL",
			"The provider cannot create the Satounki API client as there is an unknown configuration value for the Base URL. "+
				"Either target apply the source of the value first, set the value statically in the configuration, or use the SATOUNKI_BASE_URL environment variable.",
		)
	}

	if config.APIToken.IsUnknown() {
		resp.Diagnostics.AddAttributeError(
			path.Root("apiToken"),
			"Unknown Satounki API Token",
			"The provider cannot create the Satounki API client as there is an unknown configuration value for the API token. "+
				"Either target apply the source of the value first, set the value statically in the configuration, or use the SATOUNKI_API_TOKEN environment variable.",
		)
	}

	if resp.Diagnostics.HasError() {
		return
	}

	// Default values to environment variables, but override
	// with Terraform configuration value if set.

	baseURL := os.Getenv("SATOUNKI_BASE_URL")
	apiToken := os.Getenv("SATOUNKI_API_TOKEN")

	if !config.BaseURL.IsNull() {
		baseURL = config.BaseURL.ValueString()
	}

	if !config.APIToken.IsNull() {
		apiToken = config.APIToken.ValueString()
	}

	// If any of the expected configurations are missing, return
	// errors with provider-specific guidance.

	if baseURL == "" {
		resp.Diagnostics.AddAttributeError(
			path.Root("baseURL"),
			"Missing Satounki API Base URL",
			"The provider cannot create the Satounki API client as there is a missing or empty value for the API Base URL. "+
				"Set the baseURL value in the configuration or use the SATOUNKI_BASE_URL environment variable. "+
				"If either is already set, ensure the value is not empty.",
		)
	}

	if apiToken == "" {
		resp.Diagnostics.AddAttributeError(
			path.Root("apiToken"),
			"Missing Satounki API Token",
			"The provider cannot create the Satounki API client as there is a missing or empty value for the API token. "+
				"Set the apiToken value in the configuration or use the SATOUNKI_API_TOKEN environment variable. "+
				"If either is already set, ensure the value is not empty.",
		)
	}

	if resp.Diagnostics.HasError() {
		return
	}

	ctx = tflog.SetField(ctx, "satounki_base_url", baseURL)
	ctx = tflog.SetField(ctx, "satounki_api_token", apiToken)
	ctx = tflog.MaskFieldValuesWithFieldKeys(ctx, "satounki_api_token")

	tflog.Debug(ctx, "Creating Satounki client")

	client := *satounki.New(
		apiToken,
		baseURL,
		fmt.Sprintf("terraform-provider-satounki-%s", p.version),
		cleanhttp.DefaultClient(),
	)

	// Make the HashiCups client available during DataSource and Resource
	// type Configure methods.
	resp.DataSourceData = client
	resp.ResourceData = client

	tflog.Info(ctx, "Configured Satounki client", map[string]any{"success": true})
}

// DataSources defines the data sources implemented in the provider.
func (p *satounkiProvider) DataSources(_ context.Context) []func() datasource.DataSource {
	return []func() datasource.DataSource{}
}

// Resources defines the resources implemented in the provider.
func (p *satounkiProvider) Resources(_ context.Context) []func() resource.Resource {
	return []func() resource.Resource{
		NewAWSAccountResource,
		NewCloudflareAccountResource,
		NewGcpProjectResource,
		NewPolicyResource,
		NewUserAliasesResource,
		NewUserRolesResource,
	}
}
