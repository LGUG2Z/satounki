package provider

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
	"satounki"
	"time"
)

func (d gcpProjectResourceData) PostBody() satounki.SettingsGcpProjectPostBody {
	return satounki.SettingsGcpProjectPostBody{
		Project:               d.Project.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *gcpProjectResourceData) PostResponse(r satounki.SettingsGcpProjectPostResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Project = types.StringValue(r.Project)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d gcpProjectResourceData) PutBody() satounki.SettingsGcpProjectPutBody {
	return satounki.SettingsGcpProjectPutBody{
		Project:               d.Project.ValueString(),
		AdminApprovalRequired: d.AdminApprovalRequired.ValueBool(),
		ApprovalsRequired:     d.ApprovalsRequired.ValueInt64(),
	}
}

func (d *gcpProjectResourceData) PutResponse(r satounki.SettingsGcpProjectPutResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Project = types.StringValue(r.Project)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}

func (d *gcpProjectResourceData) GetResponse(r satounki.SettingsGcpProjectGetResponse) {
	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Project = types.StringValue(r.Project)
	d.AdminApprovalRequired = types.BoolValue(r.AdminApprovalRequired)
	d.ApprovalsRequired = types.Int64Value(r.ApprovalsRequired)
}
