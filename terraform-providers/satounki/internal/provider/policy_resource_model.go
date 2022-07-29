package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
)

func (d policyResourceData) PostBody() satounki.PolicyPostBody {
	var aws []string
	var cloudflare []satounki.CloudflareRole
	var gcp []string

	for _, r := range d.Aws {
		aws = append(aws, r.Value)
	}

	for _, r := range d.Cloudflare {
		cloudflare = append(cloudflare, satounki.CloudflareRole(r.Value))
	}

	for _, r := range d.Gcp {
		gcp = append(gcp, r.Value)
	}

	return satounki.PolicyPostBody{
		Aws:         aws,
		Cloudflare:  cloudflare,
		Description: d.Description.Value,
		Gcp:         gcp,
		Name:        d.Name.Value,
	}
}

func (d *policyResourceData) PostResponse(r satounki.PolicyPostResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.String{Value: p})
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.String{Value: string(p)})
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.String{Value: p})
	}

	d.ID = types.String{Value: r.ID}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Description = types.String{Value: r.Description}
	d.Name = types.String{Value: r.Name}
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp
}

func (d policyResourceData) PutBody() satounki.PolicyPutBody {
	var aws []string
	var cloudflare []satounki.CloudflareRole
	var gcp []string

	for _, r := range d.Aws {
		aws = append(aws, r.Value)
	}

	for _, r := range d.Cloudflare {
		cloudflare = append(cloudflare, satounki.CloudflareRole(r.Value))
	}

	for _, r := range d.Gcp {
		gcp = append(gcp, r.Value)
	}

	return satounki.PolicyPutBody{
		Aws:         aws,
		Cloudflare:  cloudflare,
		Description: d.Description.Value,
		Gcp:         gcp,
		Name:        d.Name.Value,
	}

}

func (d *policyResourceData) PutResponse(r satounki.PolicyPutResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.String{Value: p})
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.String{Value: string(p)})
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.String{Value: p})
	}

	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Description = types.String{Value: r.Description}
	d.Name = types.String{Value: r.Name}
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp
}

func (d *policyResourceData) GetResponse(r satounki.PolicyGetResponse) {
	var aws []types.String
	var cloudflare []types.String
	var gcp []types.String

	for _, p := range r.Aws {
		aws = append(aws, types.String{Value: p})
	}

	for _, p := range r.Cloudflare {
		cloudflare = append(cloudflare, types.String{Value: string(p)})
	}

	for _, p := range r.Gcp {
		gcp = append(gcp, types.String{Value: p})
	}

	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}
	d.Description = types.String{Value: r.Description}
	d.Name = types.String{Value: r.Name}
	d.Aws = aws
	d.Cloudflare = cloudflare
	d.Gcp = gcp

}
