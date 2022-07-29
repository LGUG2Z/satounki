pub const POST_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Post(body: satounki.{{ name}}PostBody): Promise<IRestResponse<satounki.{{ name }}PostResponse | satounki.ErrorResponse>> {
    return await this.client.create("/v1{{ post }}", body);
}
"#;
