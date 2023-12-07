package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d cloudflareAccountResourceData) PostRequest() satounki.SettingsCloudflareAccountPostRequest {
	return satounki.SettingsCloudflareAccountPostRequest{
		Account:               d.Account.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *cloudflareAccountResourceData) PostResponse(r satounki.SettingsCloudflareAccountPostResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d cloudflareAccountResourceData) PutRequest() satounki.SettingsCloudflareAccountPutRequest {
	return satounki.SettingsCloudflareAccountPutRequest{
		Account:               d.Account.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *cloudflareAccountResourceData) PutResponse(r satounki.SettingsCloudflareAccountPutResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d *cloudflareAccountResourceData) GetResponse(r satounki.SettingsCloudflareAccountGetResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Account = types.StringValue(r.Account)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}
