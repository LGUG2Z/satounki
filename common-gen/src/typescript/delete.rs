pub const DELETE_TEMPLATE_TS: &str = r#"
async {{ name|tsify }}Delete(id: string): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1{{ delete|replace("%s", "${id}") }}`);
}
"#;
