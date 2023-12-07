package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d gcpProjectResourceData) PostRequest() satounki.SettingsGcpProjectPostRequest {
	return satounki.SettingsGcpProjectPostRequest{
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

func (d gcpProjectResourceData) PutRequest() satounki.SettingsGcpProjectPutRequest {
	return satounki.SettingsGcpProjectPutRequest{
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
