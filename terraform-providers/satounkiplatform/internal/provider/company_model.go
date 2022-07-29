package provider

import (
	satounki "satounki-platform"
	"strconv"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d companyResourceData) PostBody() (satounki.CompanyPostBody, error) {
	return satounki.CompanyPostBody{
		Domain:            d.Domain.Value,
		Name:              d.Name.Value,
		RootUserEmail:     d.RootUserEmail.Value,
		RootUserFirstName: &d.RootUserFirstName.Value,
		RootUserLastName:  &d.RootUserLastName.Value,
	}, nil
}

func (d *companyResourceData) PostResponse(r satounki.CompanyPostResponse) {
	d.ID = types.String{Value: strconv.FormatInt(r.ID, 10)}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Name = types.String{Value: r.Name}
	d.Domain = types.String{Value: r.Domain}
	d.RootUserEmail = types.String{Value: r.RootUserEmail}
	d.RootUserFirstName = types.String{Value: *r.RootUserFirstName}
	d.RootUserLastName = types.String{Value: *r.RootUserLastName}
}

func (d companyResourceData) PutBody() (satounki.CompanyPutBody, error) {
	return satounki.CompanyPutBody{
		Domain:            d.Domain.Value,
		Name:              d.Name.Value,
		RootUserEmail:     d.RootUserEmail.Value,
		RootUserFirstName: &d.RootUserFirstName.Value,
		RootUserLastName:  &d.RootUserLastName.Value,
	}, nil
}

func (d *companyResourceData) PutResponse(r satounki.CompanyPutResponse) {
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Name = types.String{Value: r.Name}
	d.Domain = types.String{Value: r.Domain}
	d.RootUserEmail = types.String{Value: r.RootUserEmail}
	d.RootUserFirstName = types.String{Value: *r.RootUserFirstName}
	d.RootUserLastName = types.String{Value: *r.RootUserLastName}
}

func (d *companyResourceData) GetResponse(r satounki.CompanyGetResponse) {
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Name = types.String{Value: r.Name}
	d.Domain = types.String{Value: r.Domain}
	d.RootUserEmail = types.String{Value: r.RootUserEmail}
	d.RootUserFirstName = types.String{Value: *r.RootUserFirstName}
	d.RootUserLastName = types.String{Value: *r.RootUserLastName}
}
