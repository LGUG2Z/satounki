pub const GET_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Get() ({{ name }}GetResponse, *ErrorResponse, error) {
	url := fmt.Sprintf("%s{{ get }}", api.BaseURL)

	resp, err := api.httpClient.Get(url)
	if err != nil {
		return {{ name }}GetResponse{}, nil, err
	}

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(resp.Body)

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return {{ name }}GetResponse{}, nil, err
	}

	if resp.StatusCode >= http.StatusOK && resp.StatusCode <= http.StatusPartialContent {
		response, err := Unmarshal{{ name }}GetResponse(respBody)
		if err != nil {
			return {{ name }}GetResponse{}, nil, err
		}

		return response, nil, nil
	} else {
		response, err := UnmarshalErrorResponse(respBody)
		if err != nil {
			return {{ name }}GetResponse{}, nil, err
		}

		return {{ name }}GetResponse{}, &response, errors.New(response.Error)
	}
}
"#;
