package provider

import (
	"github.com/hashicorp/terraform-plugin-framework/types"
	"satounki"
	"time"
)

func (d policyResourceData) PostBody() satounki.PolicyPostBody {
	var aws []string
	var cloudflare []satounki.CloudflareRole
	var gcp []string

	for _, r := range d.Aws {
		aws = append(aws, r.ValueString())
	}

	for _, r := range d.Cloudflare {
		cloudflare = append(cloudflare, satounki.CloudflareRole(r.ValueString()))
	}

	for _, r := range d.Gcp {
		gcp = append(gcp, r.ValueString())
	}

	return satounki.PolicyPostBody{
		Aws:         aws,
		Cloudflare:  cloudflare,
		Description: d.Description.ValueString(),
		Gcp:         gcp,
		Name:        d.Name.ValueString(),
	}
}

func (d *policyResourceData) PostResponse(r satounki.PolicyPostResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.StringValue(p))
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.StringValue(string(p)))
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.StringValue(p))
	}

	d.ID = types.StringValue(r.ID)
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Description = types.StringValue(r.Description)
	d.Name = types.StringValue(r.Name)
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp
}

func (d policyResourceData) PutBody() satounki.PolicyPutBody {
	var aws []string
	var cloudflare []satounki.CloudflareRole
	var gcp []string

	for _, r := range d.Aws {
		aws = append(aws, r.ValueString())
	}

	for _, r := range d.Cloudflare {
		cloudflare = append(cloudflare, satounki.CloudflareRole(r.ValueString()))
	}

	for _, r := range d.Gcp {
		gcp = append(gcp, r.ValueString())
	}

	return satounki.PolicyPutBody{
		Aws:         aws,
		Cloudflare:  cloudflare,
		Description: d.Description.ValueString(),
		Gcp:         gcp,
		Name:        d.Name.ValueString(),
	}

}

func (d *policyResourceData) PutResponse(r satounki.PolicyPutResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.StringValue(p))
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.StringValue(string(p)))
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.StringValue(p))
	}

	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Description = types.StringValue(r.Description)
	d.Name = types.StringValue(r.Name)
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp
}

func (d *policyResourceData) GetResponse(r satounki.PolicyGetResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.StringValue(p))
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.StringValue(string(p)))
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.StringValue(p))
	}

	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))
	d.Description = types.StringValue(r.Description)
	d.Name = types.StringValue(r.Name)
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp
}
