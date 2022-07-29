pub const GET_ID_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Get(id: string): Promise<IRestResponse<satounki.{{ name }}GetResponse | satounki.ErrorResponse>> {
    return await this.client.get(`/v1{{ get|replace("%s", "${id}") }}`);
}
"#;
