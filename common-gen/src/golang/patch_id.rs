pub const PATCH_ID_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Patch(id string) error {
	url := fmt.Sprintf("%s{{ patch }}", api.BaseURL, id)

	req, err := http.NewRequest(http.MethodPatch,
		url,
		nil,
	)

	req.Header.Add("Content-Type", "application/json")
	resp, err := api.httpClient.Do(req)
	if err != nil {
		return err
	}

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(resp.Body)

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	if resp.StatusCode >= http.StatusOK && resp.StatusCode <= http.StatusPartialContent {
		return nil
	} else {
		response, err := UnmarshalErrorResponse(respBody)
		if err != nil {
			return err
		}

		return errors.New(response.Error)
	}
}
"#;
