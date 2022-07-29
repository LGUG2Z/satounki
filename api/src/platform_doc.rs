use actix_web::http::StatusCode;
// use common::*;
use common_platform::CompaniesGetResponse;
use common_platform::Company;
use common_platform::CompanyGetResponse;
use common_platform::CompanyPostBody;
use common_platform::CompanyPostResponse;
use common_platform::CompanyPutBody;
use common_platform::CompanyPutResponse;
use common_platform::ErrorResponse;
use common_platform::PlatformToken;
use common_platform::PlatformTokenGetResponse;
use common_platform::PlatformTokenPutResponse;
use utoipa::openapi::security::Http;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

use crate::company;
use crate::platform_token;

pub fn ex(code: StatusCode) -> ErrorResponse {
    ErrorResponse {
        code: code.as_u16(),
        error: code.to_string().replace(&format!("{} ", code.as_u16()), ""),
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        company::companies_get,
        company::company_get,
        company::company_post,
        company::company_put,
        company::company_delete,
        platform_token::platform_token_get,
        platform_token::platform_token_put,
    ),
    components(
        responses(
            CompaniesGetResponse,
            CompanyGetResponse,
            CompanyPostResponse,
            CompanyPutResponse,
            PlatformTokenGetResponse,
            PlatformTokenPutResponse,
        ),
        schemas(
            // these need to be duplicated here
            CompaniesGetResponse,
            CompanyGetResponse,
            CompanyPostResponse,
            CompanyPutResponse,
            PlatformTokenGetResponse,
            PlatformTokenPutResponse,

            Company,
            CompanyPostBody,
            CompanyPutBody,
            ErrorResponse,
            PlatformToken,
        )
    )
)]
pub struct PlatformDoc;
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "platform_token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );

        openapi.info.title = String::from("Satounki Platform API");
        openapi.info.description = Option::from(String::from(
            "Used for managing company configuration via Terraform",
        ));
    }
}
