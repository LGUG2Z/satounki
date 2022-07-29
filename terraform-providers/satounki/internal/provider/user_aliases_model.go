package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/resource"
)

func (d userAliasesResourceData) PostBody() satounki.UserAliasesPostBody {
	return satounki.UserAliasesPostBody{
		Aws:        &d.Aws.Value,
		Cloudflare: &d.Cloudflare.Value,
		Gcp:        &d.Gcp.Value,
	}
}

func (d *userAliasesResourceData) PostResponse(r satounki.UserAliasesPostResponse) {
	d.ID = types.String{Value: resource.UniqueId()}
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.String{Value: *r.Aws}
	} else {
		d.Aws = types.String{Null: true}
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.String{Value: *r.Cloudflare}
	} else {
		d.Cloudflare = types.String{Null: true}
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.String{Value: *r.Gcp}
	} else {
		d.Gcp = types.String{Null: true}
	}
}

func (d userAliasesResourceData) PutBody() satounki.UserAliasesPutBody {
	return satounki.UserAliasesPutBody{
		Aws:        &d.Aws.Value,
		Cloudflare: &d.Cloudflare.Value,
		Gcp:        &d.Gcp.Value,
	}
}

func (d *userAliasesResourceData) PutResponse(r satounki.UserAliasesPutResponse) {
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.String{Value: *r.Aws}
	} else {
		d.Aws = types.String{Null: true}
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.String{Value: *r.Cloudflare}
	} else {
		d.Cloudflare = types.String{Null: true}
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.String{Value: *r.Gcp}
	} else {
		d.Gcp = types.String{Null: true}
	}
}

func (d *userAliasesResourceData) GetResponse(r satounki.UserAliasesGetResponse) {
	d.LastUpdated = types.String{Value: time.Now().Format(time.RFC850)}

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.String{Value: *r.Aws}
	} else {
		d.Aws = types.String{Null: true}
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.String{Value: *r.Cloudflare}
	} else {
		d.Cloudflare = types.String{Null: true}
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.String{Value: *r.Gcp}
	} else {
		d.Gcp = types.String{Null: true}
	}
}
