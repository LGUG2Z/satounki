package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d awsAccountResourceData) PostBody() satounki.SettingsAwsAccountPostBody {
	return satounki.SettingsAwsAccountPostBody{
		Account:               d.Account.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *awsAccountResourceData) PostResponse(r satounki.SettingsAwsAccountPostResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d awsAccountResourceData) PutBody() satounki.SettingsAwsAccountPutBody {
	return satounki.SettingsAwsAccountPutBody{
		Account:               d.Account.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *awsAccountResourceData) PutResponse(r satounki.SettingsAwsAccountPutResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d *awsAccountResourceData) GetResponse(r satounki.SettingsAwsAccountGetResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Account = types.String{Value: r.Account}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}
