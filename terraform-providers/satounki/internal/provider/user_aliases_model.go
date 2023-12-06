package provider

import (
	"satounki"
	"time"

	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/id"
)

func (d userAliasesResourceData) PostBody() satounki.UserAliasesPostBody {
	return satounki.UserAliasesPostBody{
		Aws:        d.Aws.ValueStringPointer(),
		Cloudflare: d.Cloudflare.ValueStringPointer(),
		Gcp:        d.Gcp.ValueStringPointer(),
	}
}

func (d *userAliasesResourceData) PostResponse(r satounki.UserAliasesPostResponse) {
	d.ID = types.StringValue(id.UniqueId())
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.StringValue(*r.Aws)
	} else {
		d.Aws = types.StringNull()
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.StringValue(*r.Cloudflare)
	} else {
		d.Cloudflare = types.StringNull()
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.StringValue(*r.Gcp)
	} else {
		d.Gcp = types.StringNull()
	}
}

func (d userAliasesResourceData) PutBody() satounki.UserAliasesPutBody {
	return satounki.UserAliasesPutBody{
		Aws:        d.Aws.ValueStringPointer(),
		Cloudflare: d.Cloudflare.ValueStringPointer(),
		Gcp:        d.Gcp.ValueStringPointer(),
	}
}

func (d *userAliasesResourceData) PutResponse(r satounki.UserAliasesPutResponse) {
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.StringValue(*r.Aws)
	} else {
		d.Aws = types.StringNull()
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.StringValue(*r.Cloudflare)
	} else {
		d.Cloudflare = types.StringNull()
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.StringValue(*r.Gcp)
	} else {
		d.Gcp = types.StringNull()
	}
}

func (d *userAliasesResourceData) GetResponse(r satounki.UserAliasesGetResponse) {
	d.LastUpdated = types.StringValue(time.Now().Format(time.RFC850))

	if r.Aws != nil && *r.Aws != "" {
		d.Aws = types.StringValue(*r.Aws)
	} else {
		d.Aws = types.StringNull()
	}

	if r.Cloudflare != nil && *r.Cloudflare != "" {
		d.Cloudflare = types.StringValue(*r.Cloudflare)
	} else {
		d.Cloudflare = types.StringNull()
	}

	if r.Gcp != nil && *r.Gcp != "" {
		d.Gcp = types.StringValue(*r.Gcp)
	} else {
		d.Gcp = types.StringNull()
	}
}
