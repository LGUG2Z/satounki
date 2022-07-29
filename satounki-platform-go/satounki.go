package satounki

import (
	"fmt"
	"net/http"
)

type API struct {
	APIToken   string
	BaseURL    string
	UserAgent  string
	httpClient *http.Client
}

func New(token, baseURL, userAgent string, client *http.Client) *API {
	api := &API{
		APIToken:   token,
		BaseURL:    baseURL,
		UserAgent:  userAgent,
		httpClient: client,
	}

	// Fall back to http.DefaultClient if the package user does not provide
	// their own.
	if api.httpClient == nil {
		api.httpClient = http.DefaultClient
	}

	api.httpClient.Transport = &customTransport{
		token:     token,
		userAgent: userAgent,
	}

	return api
}

type customTransport struct {
	token     string
	userAgent string
}

func (t *customTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", t.token))
	req.Header.Add("User-Agent", t.userAgent)
	return http.DefaultTransport.RoundTrip(req)
}
