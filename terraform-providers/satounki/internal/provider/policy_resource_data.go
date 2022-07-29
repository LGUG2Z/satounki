// Generated by satounki/common-gen

package provider

import "github.com/hashicorp/terraform-plugin-framework/types"

// Satounki Policy definition
type policyResourceData struct {
	// Time of the last modification to this resource
	LastUpdated types.String `tfsdk:"last_updated" rustdoc:"Time of the last modification to this resource" resourcedoc:"Satounki Policy definition"`
	// UUID generated by Satounki
	ID types.String `tfsdk:"id" rustdoc:"UUID generated by Satounki" resourcedoc:"Satounki Policy definition"`
	// Succinct, descriptive name for the policy in snake_case
	Name types.String `tfsdk:"name" rustdoc:"Succinct, descriptive name for the policy in snake_case" resourcedoc:"Satounki Policy definition"`
	// Description of the permissions granted by this policy
	Description types.String `tfsdk:"description" rustdoc:"Description of the permissions granted by this policy" resourcedoc:"Satounki Policy definition"`
	// Google Cloud Platform roles associated with this policy
	Gcp []types.String `tfsdk:"gcp" rustdoc:"Google Cloud Platform roles associated with this policy" resourcedoc:"Satounki Policy definition"`
	// Amazon Web Services policy ARNs associated with this policy
	Aws []types.String `tfsdk:"aws" rustdoc:"Amazon Web Services policy ARNs associated with this policy" resourcedoc:"Satounki Policy definition"`
	// Cloudflare roles associated with this policy
	Cloudflare []types.String `tfsdk:"cloudflare" rustdoc:"Cloudflare roles associated with this policy" resourcedoc:"Satounki Policy definition"`
}
