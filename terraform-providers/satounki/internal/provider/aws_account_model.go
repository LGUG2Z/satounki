package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d awsAccountResourceData) PostRequest() satounki.SettingsAwsAccountPostRequest {
	return satounki.SettingsAwsAccountPostRequest{
		Account:               d.Account.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *awsAccountResourceData) PostResponse(r satounki.SettingsAwsAccountPostResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d awsAccountResourceData) PutRequest() satounki.SettingsAwsAccountPutRequest {
	return satounki.SettingsAwsAccountPutRequest{
		Account:               d.Account.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *awsAccountResourceData) PutResponse(r satounki.SettingsAwsAccountPutResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d *awsAccountResourceData) GetResponse(r satounki.SettingsAwsAccountGetResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}
