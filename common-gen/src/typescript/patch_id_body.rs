pub const PATCH_ID_BODY_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Patch(id: string, body: satounki.{{ name}}PatchRequest): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.update(`/v1{{ patch|replace("%s", "${id}") }}`, body);
}
"#;
