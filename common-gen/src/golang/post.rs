pub const POST_TEMPLATE_GO: &str = r#"
func (api *API) {{ name }}Post(body {{ name }}PostRequest) ({{ name }}PostResponse, *ErrorResponse, error) {
	url := fmt.Sprintf("%s{{ post }}", api.BaseURL)
	reqBody, err := json.Marshal(&body)
	if err != nil {
		return {{ name }}PostResponse{}, nil, err
	}

	resp, err := api.httpClient.Post(
		url,
		"application/json",
		bytes.NewBuffer(reqBody),
	)

	if err != nil {
		return {{ name }}PostResponse{}, nil, err
	}

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(resp.Body)

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return {{ name }}PostResponse{}, nil, err
	}

	if resp.StatusCode >= http.StatusOK && resp.StatusCode <= http.StatusPartialContent {
		response, err := Unmarshal{{ name }}PostResponse(respBody)
		if err != nil {
			return {{ name }}PostResponse{}, nil, err
		}

		return response, nil, nil
	} else {
		response, err := UnmarshalErrorResponse(respBody)
		if err != nil {
			return {{ name }}PostResponse{}, nil, err
		}

		return {{ name }}PostResponse{}, &response, errors.New(response.Error)
	}
}
"#;
