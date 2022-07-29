pub const PUT_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Put() ({{ name }}PutResponse, *ErrorResponse, error) {
	url := fmt.Sprintf("%s{{ put }}", api.BaseURL)

	req, err := http.NewRequest(http.MethodPut, url, nil)
	if err != nil {
		return {{ name }}PutResponse{}, nil, err
	}

	req.Header.Add("Content-Type", "application/json")
	resp, err := api.httpClient.Do(req)
	if err != nil {
		return {{ name }}PutResponse{}, nil, err
	}

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(resp.Body)

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return {{ name }}PutResponse{}, nil, err
	}

	if resp.StatusCode >= http.StatusOK && resp.StatusCode <= http.StatusPartialContent {
		response, err := Unmarshal{{ name }}PutResponse(respBody)
		if err != nil {
			return {{ name }}PutResponse{}, nil, err
		}

		return response, nil, nil
	} else {
		response, err := UnmarshalErrorResponse(respBody)
		if err != nil {
			return {{ name }}PutResponse{}, nil, err
		}

		return {{ name }}PutResponse{}, &response, errors.New(response.Error)
	}
}
"#;
