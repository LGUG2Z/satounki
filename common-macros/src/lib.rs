pub use derive_more::Deref;
pub use display_json::DisplayAsJsonPretty;
pub use paste::paste;
pub use schemars::JsonSchema;
pub use serde::Deserialize;
pub use serde::Serialize;
pub use utoipa::ToResponse;
pub use utoipa::ToSchema;

/// Macro to generate API wrapper structs
///
/// # Format:
///
/// ```text
/// #[HTTP_METHOD] ROUTE(REQUEST_TYPE?) -> RESPONSE_TYPE
/// ```
///
/// # Example:
///
/// ```rust
/// use common_macros::route_request_response;
///
/// pub struct Policy { name: String }
/// route_request_response! {
///     #[Post] Policy(Policy) -> Policy,
///     // pub struct PostPolicyBody(pub Policy);
///     #[Put] Policy(Policy) -> Policy,
///     // pub struct PutPolicyBody(pub Policy);
///     #[Get] Policy() -> Policy,
///     // pub struct GetPolicyResponse(pub Policy);
///     #[Get] Policies() -> Vec<Policy>,
///     // pub struct GetPoliciesResponse(pub Vec<Policy>);
/// }
/// ```
#[macro_export]
macro_rules! route_request_response {
    ($(#[$method:ident] $route:ident($( $req:ident )?) -> $ret:ty) ,+ $(,)?) => {
        $(
            $crate::paste! {
                #[derive(
                    Debug,
                    Clone,
                    $crate::Deref,
                    $crate::Serialize,
                    $crate::Deserialize,
                    $crate::DisplayAsJsonPretty,
                    $crate::JsonSchema,
                    $crate::ToSchema,
                    $crate::ToResponse,
                )]
                pub struct [< $route $method Response >](pub $ret);

                $(
                    #[derive(
                        Debug,
                        Clone,
                        $crate::Deref,
                        $crate::Serialize,
                        $crate::Deserialize,
                        $crate::DisplayAsJsonPretty,
                        $crate::JsonSchema,
                        $crate::ToSchema,
                    )]
                    pub struct [< $route $method Body >](pub $req);
                )?
            }
        )+

    };

}

/// Macro to generate response body wrapper structs
///
/// # Format:
///
/// ```text
/// #[HTTP_METHOD] WRAPPER_PREFIX -> WRAPPED TYPE
/// ```
///
/// # Example:
///
/// ```rust
/// pub struct Policy { name: String }
/// body! {
///     #[Get] Policy -> Policy
///     // pub struct GetPolicyResponse(pub Policy);
///     #[Get] Policies -> Vec<Policy>
///     // pub struct GetPoliciesResponse(pub Vec<Policies>);
/// }
/// ```
#[macro_export]
macro_rules! response {
    ($(#[$method:ident] $kind:ident -> $ret:ty) ,+ $(,)?) => {
        $(
            $crate::paste! {
                #[derive(
                    Debug,
                    Clone,
                    $crate::Deref,
                    $crate::Serialize,
                    $crate::Deserialize,
                    $crate::DisplayAsJsonPretty,
                    $crate::JsonSchema,
                    $crate::ToSchema,
                    $crate::ToResponse,
                )]
                pub struct [< $kind $method Response >](pub $ret);
            }
        )+

    };
}

/// Macro to generate request body wrapper structs
///
/// # Format:
///
/// ```text
/// #[HTTP_METHOD] WRAPPER_PREFIX -> WRAPPED TYPE
/// ```
///
/// # Example:
///
/// ```rust
/// pub struct Policy { name: String }
/// body! {
///     #[Post] Policy -> Policy
///     // pub struct PostPolicyResponse(pub Policy);
///     #[Post] Policies -> Vec<Policy>
///     // pub struct PostPoliciesResponse(pub Vec<Policies>);
/// }
/// ```
#[macro_export]
macro_rules! body {
    ($(#[$method:ident] $kind:ident -> $ret:ty) ,+ $(,)?) => {
        $(
            $crate::paste! {
                #[derive(
                    Debug,
                    Clone,
                    $crate::Deref,
                    $crate::Serialize,
                    $crate::Deserialize,
                    $crate::DisplayAsJsonPretty,
                    $crate::JsonSchema,
                    $crate::ToSchema,
                )]
                pub struct [< $kind $method Body >](pub $ret);
            }
        )+

    };
}

/// Macro to generate a Vec of Golang member definitions for a Rust struct with tfsdk annotations
///
/// Can be applied to both Structs with members, and Enums
///
/// # Example
/// ```rust
/// #[macro_rules_derive(terraform_resource!)]
/// pub struct Policy { name: String };
/// assert!(Policy::terraform_resource_members, vec!["Name types.String `tfsdk:\"name\"".to_string()])
/// ```
#[macro_export]
macro_rules! terraform_resource {
(
   $(#[$enum_meta:meta])*
   $enum_vis:vis enum $EnumName:ident {
       $(
           $(#[$field_meta:meta])*
           $field_name:ident
       ),* $(,)?
   }
) => (
    ::paste::paste! {
        impl $EnumName {
            pub fn terraform_resource_members() -> (&'static str, Vec<String>) {
                let mut enum_docs: Vec<&str> = Vec::from(
                    [
                        $(
                            stringify!($enum_meta),
                        )*
                    ]
                );

                enum_docs.retain(|f| f.starts_with("doc"));

                let enum_docs: Vec<_> = enum_docs
                    .iter_mut()
                    .map(|f| f.trim_start_matches("doc ="))
                    .map(|f| f.trim_start())
                    .map(|f| f.trim_start_matches("r\" "))
                    .map(|f| f.trim_start_matches("\" "))
                    .map(|f| f.trim_end_matches('"'))
                    .map(|f| f.trim_start_matches("r#\" "))
                    .map(|f| f.trim_end_matches("\"# "))
                    .collect();

                let enum_doc = enum_docs.first().unwrap_or(&stringify!($EnumName));

                let doc_string: String = stringify!([<$EnumName:snake>]).split('_').collect::<Vec<&str>>().join(" ");
                let s = doc_string.as_str();
                let doc_string = s[0..1].to_uppercase() + &s[1..];

                let mut members = Vec::from(
                    [
                        format!("// {}\n{} {} `tfsdk:\"{}\" rustdoc:\"{}\" resourcedoc:\"{}\"`",
                            "Time of the last modification to this resource",
                            "LastUpdated",
                            "types.String",
                            "last_updated",
                            "Time of the last modification to this resource",
                            enum_doc,
                        ),
                        "// Terraform-generated resource ID\nID types.String `tfsdk:\"id\" rustdoc:\"Terraform-generated resource ID\"`".to_string(),
                        format!(
                            "// {}s\n{}s []types.String `tfsdk:\"{}s\" rustdoc:\"{}s\" resourcedoc:\"{}\"`",
                            doc_string,
                            stringify!([<$EnumName:camel>]),
                            stringify!([<$EnumName:snake>]),
                            doc_string,
                            enum_doc,
                        )
                    ]
                );

                match stringify!($EnumName) {
                    "AccessRole" => {
                        members.insert(
                            2,
                            format!(
                                "// Email address registered with Satounki\nEmail types.String `tfsdk:\"email\" rustdoc:\"Email address registered with Satounki\" resourcedoc:\"{}\"`",
                                enum_doc,
                            )
                        );
                    },
                    _ => {},
                }

                (enum_doc, members)
            }
        }
    }
);
(
   $(#[$struct_meta:meta])*
   $struct_vis:vis struct $StructName:ident {
       $(
           $(#[$field_meta:meta])*
           $field_vis:vis $field_name:ident : $field_ty:ty
       ),* $(,)?
   }
) => (
    ::paste::paste! {
        impl $StructName {
            pub fn terraform_resource_members() -> (&'static str, Vec<String>) {
                let mut struct_docs: Vec<&str> = Vec::from(
                    [
                        $(
                            stringify!($struct_meta),
                        )*
                    ]
                );


                struct_docs.retain(|f| f.starts_with("doc"));

                let mut field_docs: Vec<&str> = Vec::from(
                    [
                        $(
                            $(
                                stringify!($field_meta),
                            )*
                        )*
                    ]
                );

                field_docs.retain(|f| f.starts_with("doc"));

                let struct_docs: Vec<_> = struct_docs
                    .iter_mut()
                    .map(|f| f.trim_start_matches("doc ="))
                    .map(|f| f.trim_start())
                    .map(|f| f.trim_start_matches("r\" "))
                    .map(|f| f.trim_start_matches("\" "))
                    .map(|f| f.trim_end_matches('"'))
                    .map(|f| f.trim_start_matches("r#\" "))
                    .map(|f| f.trim_end_matches("\"# "))
                    .collect();

                let field_docs: Vec<_> = field_docs
                    .iter_mut()
                    .map(|f| f.trim_start_matches("doc ="))
                    .map(|f| f.trim_start())
                    .map(|f| f.trim_start_matches("r\" "))
                    .map(|f| f.trim_end_matches('"'))
                    .map(|f| f.trim_start_matches("r#\" "))
                    .map(|f| f.trim_end_matches("\"# "))
                    .collect();

                let resource = Vec::from(
                    [
                       $(
                           (stringify!([<$field_name:camel>]), $crate::terraform_type!($field_ty, $field_name), stringify!([< $field_name:snake >])),
                       )*
                    ]
                );

                let struct_doc = struct_docs.first().unwrap_or(&stringify!($StructName));

                let mut members = vec![
                    format!("// {}\n{} {} `tfsdk:\"{}\" rustdoc:\"{}\" resourcedoc:\"{}\"`",
                        "Time of the last modification to this resource",
                        "LastUpdated",
                        "types.String",
                        "last_updated",
                        "Time of the last modification to this resource",
                        struct_doc,
                    )
                ];

                let mut has_id = false;
                let mut needs_identifier = false;
                let mut identifier = ("Camel", "snake", "Documentation");

                for (i, (member, kind, snake)) in resource.into_iter().enumerate() {
                    match stringify!($StructName) {
                        "UserAliases" => {
                            needs_identifier = true;
                            identifier = ("Email", "email", "Email address registered with Satounki");
                        },
                        _ => {},
                    }

                    members.push(
                        format!("// {}\n{} {} `tfsdk:\"{}\" rustdoc:\"{}\" resourcedoc:\"{}\"`",
                            field_docs.get(i).unwrap_or(&member),
                            match member {
                                "Id" => {
                                    has_id = true;
                                    "ID"
                                },
                                member => member,
                            },
                            kind,
                            snake,
                            field_docs.get(i).unwrap_or(&member),
                            struct_doc,
                        )
                    );
                }

                if needs_identifier {
                    members.insert(
                        1,
                        format!("// {}\n{} types.String `tfsdk:\"{}\" rustdoc:\"{}\" resourcedoc:\"{}\"`",
                            identifier.2,
                            identifier.0,
                            identifier.1,
                            identifier.2,
                            struct_doc,
                        )
                    );
                }

                if !has_id {
                    members.insert(1, format!(
                        "// Terraform-generated resource ID\nID types.String `tfsdk:\"id\" rustdoc:\"Terraform-generated resource ID\" resourcedoc:\"{}\"`",
                        struct_doc,
                    ));
                }


                (struct_doc, members)
            }
        }
    }
);
}

#[macro_export]
macro_rules! terraform_type {
    ($kind:ty, $name:ident) => {
        match stringify!($kind) {
            _ if stringify!($name) == "id" => "types.String",
            "String" => "types.String",
            "i32" | "i64" => "types.Int64",
            "bool" => "types.Bool",
            "DateTime<Utc>" => "types.String",
            "Option<String>" => "types.String",
            "Option<Vec<GcpRole>>" => "[]types.String",
            "Option<Vec<AwsPolicy>>" => "[]types.String",
            "Option<Vec<CloudflareRole>>" => "[]types.String",
            x => panic!(
                "this case has not yet been handled: {} for {}",
                x,
                stringify!($name)
            ),
        }
    };
}

/// Macro to generate a New<Struct> for Diesel insertions without an 'id' field
///
/// All struct and field metadata is kept; documentation, serde attributes etc.
///
/// # Example:
///
/// ```rust
/// #[macro_rules_derive(new_resource!)]
/// pub struct Policy { id: i32, name: String }
///
/// // The macro will create the following struct:
/// // pub struct NewPolicy { name: String }
/// ```
#[macro_export]
macro_rules! new_resource {
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $StructName:ident {
            // We want to extract the ID field from the repetition
            $(#[$_id_meta:meta])*
            pub id : $_id_type:ty,
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => (
        ::paste::paste! {
            $(#[$struct_meta])*
            $struct_vis struct [< New $StructName >] {
                $(
                    $(#[$field_meta])*
                    $field_vis $field_name: $field_ty,
                )*
            }
        }
    );
}
