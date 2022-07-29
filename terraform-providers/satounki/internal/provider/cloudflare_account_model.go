package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d cloudflareAccountResourceData) PostBody() satounki.SettingsCloudflareAccountPostBody {
	return satounki.SettingsCloudflareAccountPostBody{
		Account:               d.Account.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *cloudflareAccountResourceData) PostResponse(r satounki.SettingsCloudflareAccountPostResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d cloudflareAccountResourceData) PutBody() satounki.SettingsCloudflareAccountPutBody {
	return satounki.SettingsCloudflareAccountPutBody{
		Account:               d.Account.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *cloudflareAccountResourceData) PutResponse(r satounki.SettingsCloudflareAccountPutResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d *cloudflareAccountResourceData) GetResponse(r satounki.SettingsCloudflareAccountGetResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}
