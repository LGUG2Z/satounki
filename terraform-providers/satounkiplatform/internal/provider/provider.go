package provider

import (
	"context"
	"fmt"
	"os"
	satounki "satounki-platform"

	"github.com/hashicorp/go-cleanhttp"

	"github.com/hashicorp/terraform-plugin-framework/diag"
	"github.com/hashicorp/terraform-plugin-framework/tfsdk"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure provider defined types fully satisfy framework interfaces
var _ tfsdk.Provider = &provider{}

// provider satisfies the tfsdk.Provider interface and usually is included
// with all Resource and DataSource implementations.
type provider struct {
	api satounki.API

	// configured is set to true at the end of the Configure method.
	// This can be used in Resource and DataSource implementations to verify
	// that the provider was previously configured.
	configured bool

	// version is set to the provider version on release, "dev" when the
	// provider is built and ran locally, and "test" when running acceptance
	// testing.
	version string
}

// providerData can be used to store data from the Terraform configuration.
type providerData struct {
	APIToken types.String `tfsdk:"api_token"`
	BaseURL  types.String `tfsdk:"base_url"`
}

func (p *provider) Configure(ctx context.Context, req tfsdk.ConfigureProviderRequest, resp *tfsdk.ConfigureProviderResponse) {
	var config providerData
	diags := req.Config.Get(ctx, &config)
	resp.Diagnostics.Append(diags...)

	if resp.Diagnostics.HasError() {
		return
	}

	var apiToken string
	var baseURL string

	if config.APIToken.Null {
		apiToken = os.Getenv("SATOUNKIPLATFORM_API_TOKEN")
	} else {
		apiToken = config.APIToken.Value
	}

	if config.BaseURL.Null {
		baseURL = "http://localhost:8080/platform"
	} else {
		baseURL = config.BaseURL.Value
	}

	if apiToken == "" {
		resp.Diagnostics.AddError("Unable to create Satounki Platform API client",
			"A Platform API Token must be provided to instantiate the Satounki Platform API client",
		)

		return
	}

	p.api = *satounki.New(
		apiToken,
		baseURL,
		fmt.Sprintf("terraform-provider-satounkiplatform-%s", p.version),
		cleanhttp.DefaultClient(),
	)

	p.configured = true
}

func (p *provider) GetResources(ctx context.Context) (map[string]tfsdk.ResourceType, diag.Diagnostics) {
	return map[string]tfsdk.ResourceType{
		"satounkiplatform_company": companyResourceType{},
	}, nil
}

func (p *provider) GetDataSources(ctx context.Context) (map[string]tfsdk.DataSourceType, diag.Diagnostics) {
	return map[string]tfsdk.DataSourceType{}, nil
}

func (p *provider) GetSchema(ctx context.Context) (tfsdk.Schema, diag.Diagnostics) {
	return tfsdk.Schema{
		Attributes: map[string]tfsdk.Attribute{
			"api_token": {
				Type:                types.StringType,
				Description:         "A Platform Token with the 'write' scope",
				MarkdownDescription: "Platform Token",
				Required:            true,
				Sensitive:           true,
			},
			"base_url": {
				Type:                types.StringType,
				Description:         "Satounki Platform API Base URL",
				MarkdownDescription: "Base URL",
				Optional:            true,
				Sensitive:           false,
			},
		},
	}, nil
}

func New(version string) func() tfsdk.Provider {
	return func() tfsdk.Provider {
		return &provider{
			version: version,
		}
	}
}

// convertProviderType is a helper function for NewResource and NewDataSource
// implementations to associate the concrete provider type. Alternatively,
// this helper can be skipped and the provider type can be directly type
// asserted (e.g. provider: in.(*provider)), however using this can prevent
// potential panics.
func convertProviderType(in tfsdk.Provider) (provider, diag.Diagnostics) {
	var diags diag.Diagnostics

	p, ok := in.(*provider)

	if !ok {
		diags.AddError(
			"Unexpected Provider Instance Type",
			fmt.Sprintf("While creating the data source or resource, an unexpected provider type (%T) was received. This is always a bug in the provider code and should be reported to the provider developers.", p),
		)
		return provider{}, diags
	}

	if p == nil {
		diags.AddError(
			"Unexpected Provider Instance Type",
			"While creating the data source or resource, an unexpected empty provider instance was received. This is always a bug in the provider code and should be reported to the provider developers.",
		)
		return provider{}, diags
	}

	return *p, diags
}
