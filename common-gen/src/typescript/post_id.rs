pub const POST_ID_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Post(id: string, body: satounki.{{ name}}PostBody): Promise<IRestResponse<satounki.{{ name }}PostResponse | satounki.ErrorResponse>> {
    return await this.client.create(`/v1{{ post|replace("%s", "${id}") }}`, body);
}
"#;
