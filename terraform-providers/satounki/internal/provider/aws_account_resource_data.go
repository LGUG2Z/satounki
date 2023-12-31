// Generated by satounki/common-gen

package provider

import "github.com/hashicorp/terraform-plugin-framework/types"

// Amazon Web Services account configuration
type awsAccountResourceData struct {
	// Time of the last modification to this resource
	LastUpdated types.String `tfsdk:"last_updated" rustdoc:"Time of the last modification to this resource" resourcedoc:"Amazon Web Services account configuration"`
	// UUID generated by Satounki
	ID types.String `tfsdk:"id" rustdoc:"UUID generated by Satounki" resourcedoc:"Amazon Web Services account configuration"`
	// Meaningful alias for the account to be used by Satounki users
	Account types.String `tfsdk:"account" rustdoc:"Meaningful alias for the account to be used by Satounki users" resourcedoc:"Amazon Web Services account configuration"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired types.Int64 `tfsdk:"approvals_required" rustdoc:"Number of approvals required for access requests made to the account" resourcedoc:"Amazon Web Services account configuration"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired types.Bool `tfsdk:"admin_approval_required" rustdoc:"Require additional approval by an Administrator for access requests made to the account" resourcedoc:"Amazon Web Services account configuration"`
}
