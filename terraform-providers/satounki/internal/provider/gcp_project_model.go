package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d gcpProjectResourceData) PostBody() satounki.SettingsGcpProjectPostBody {
	return satounki.SettingsGcpProjectPostBody{
		Project:               d.Project.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *gcpProjectResourceData) PostResponse(r satounki.SettingsGcpProjectPostResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Project = types.String{Value: r.Project}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d gcpProjectResourceData) PutBody() satounki.SettingsGcpProjectPutBody {
	return satounki.SettingsGcpProjectPutBody{
		Project:               d.Project.Value,
		AdminApprovalRequired: d.AdminApprovalRequired.Value,
		ApprovalsRequired:     d.ApprovalsRequired.Value,
	}
}

func (d *gcpProjectResourceData) PutResponse(r satounki.SettingsGcpProjectPutResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Project = types.String{Value: r.Project}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}

func (d *gcpProjectResourceData) GetResponse(r satounki.SettingsGcpProjectGetResponse) {
	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Project = types.String{Value: r.Project}
	d.AdminApprovalRequired = types.Bool{Value: r.AdminApprovalRequired}
	d.ApprovalsRequired = types.Int64{Value: r.ApprovalsRequired}
}
