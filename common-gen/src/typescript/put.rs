pub const PUT_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Put(): Promise<IRestResponse<satounki.{{ name }}PutResponse | satounki.ErrorResponse>> {
    return await this.client.replace("/v1{{ put }}", null);
}
"#;
