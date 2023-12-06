package provider

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
	"satounki"
	"time"
)

func (d cloudflareAccountResourceData) PostBody() satounki.SettingsCloudflareAccountPostBody {
	return satounki.SettingsCloudflareAccountPostBody{
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

func (d cloudflareAccountResourceData) PutBody() satounki.SettingsCloudflareAccountPutBody {
	return satounki.SettingsCloudflareAccountPutBody{
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
