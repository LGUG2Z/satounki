package provider

import (
	satounki "satounki-platform"
	"strconv"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d companyResourceData) PostBody() satounki.CompanyPostBody {
	return satounki.CompanyPostBody{
		Domain:            d.Domain.ValueString(),
		Name:              d.Name.ValueString(),
		RootUserEmail:     d.RootUserEmail.ValueString(),
		RootUserFirstName: d.RootUserFirstName.ValueStringPointer(),
		RootUserLastName:  d.RootUserLastName.ValueStringPointer(),
	}
}

func (d *companyResourceData) PostResponse(r satounki.CompanyPostResponse) {
	d.ID = types.StringValue(strconv.FormatInt(r.ID, 10))
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Name = types.StringValue(r.Name)
	d.Domain = types.StringValue(r.Domain)
	d.RootUserEmail = types.StringValue(r.RootUserEmail)
	d.RootUserFirstName = types.StringValue(*r.RootUserFirstName)
	d.RootUserLastName = types.StringValue(*r.RootUserLastName)
}

func (d companyResourceData) PutBody() satounki.CompanyPutBody {
	return satounki.CompanyPutBody{
		Domain:            d.Domain.ValueString(),
		Name:              d.Name.ValueString(),
		RootUserEmail:     d.RootUserEmail.ValueString(),
		RootUserFirstName: d.RootUserFirstName.ValueStringPointer(),
		RootUserLastName:  d.RootUserLastName.ValueStringPointer(),
	}
}

func (d *companyResourceData) PutResponse(r satounki.CompanyPutResponse) {
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Name = types.StringValue(r.Name)
	d.Domain = types.StringValue(r.Domain)
	d.RootUserEmail = types.StringValue(r.RootUserEmail)
	d.RootUserFirstName = types.StringValue(*r.RootUserFirstName)
	d.RootUserLastName = types.StringValue(*r.RootUserLastName)
}

func (d *companyResourceData) GetResponse(r satounki.CompanyGetResponse) {
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Name = types.StringValue(r.Name)
	d.Domain = types.StringValue(r.Domain)
	d.RootUserEmail = types.StringValue(r.RootUserEmail)
	d.RootUserFirstName = types.StringValue(*r.RootUserFirstName)
	d.RootUserLastName = types.StringValue(*r.RootUserLastName)
}
