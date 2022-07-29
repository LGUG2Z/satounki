pub const GET_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Get(): Promise<IRestResponse<satounki.{{ name }}GetResponse | satounki.ErrorResponse>> {
    return await this.client.get("/v1{{ get }}");
}
"#;
