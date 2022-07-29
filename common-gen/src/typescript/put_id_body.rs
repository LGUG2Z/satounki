pub const PUT_ID_BODY_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Put(id: string, body: satounki.{{ name}}PutBody): Promise<IRestResponse<satounki.{{ name }}PutResponse | satounki.ErrorResponse>> {
    return await this.client.replace(`/v1{{ put|replace("%s", "${id}") }}`, body);
}
"#;
