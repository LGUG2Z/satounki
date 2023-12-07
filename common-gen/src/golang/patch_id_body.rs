pub const PATCH_ID_BODY_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Patch(id string, body {{ name }}PatchRequest) error {
	url := fmt.Sprintf("%s{{ patch }}", api.BaseURL, id)
	reqBody, err := json.Marshal(&body)
	if err != nil {
		return err
	}

	req, err := http.NewRequest(http.MethodPatch,
		url,
		bytes.NewBuffer(reqBody),
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
