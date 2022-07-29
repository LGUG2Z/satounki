pub const DELETE_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Delete(id string) error {
	url := fmt.Sprintf("%s{{ delete }}", api.BaseURL, id)
	req, err := http.NewRequest(http.MethodDelete, url, nil)
	if err != nil {
		return err
	}

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
