// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    companiesGetResponse, err := UnmarshalCompaniesGetResponse(bytes)
//    bytes, err = companiesGetResponse.Marshal()
//
//    companyGetResponse, err := UnmarshalCompanyGetResponse(bytes)
//    bytes, err = companyGetResponse.Marshal()
//
//    companyPostRequest, err := UnmarshalCompanyPostRequest(bytes)
//    bytes, err = companyPostRequest.Marshal()
//
//    companyPostResponse, err := UnmarshalCompanyPostResponse(bytes)
//    bytes, err = companyPostResponse.Marshal()
//
//    companyPutRequest, err := UnmarshalCompanyPutRequest(bytes)
//    bytes, err = companyPutRequest.Marshal()
//
//    companyPutResponse, err := UnmarshalCompanyPutResponse(bytes)
//    bytes, err = companyPutResponse.Marshal()
//
//    errorResponse, err := UnmarshalErrorResponse(bytes)
//    bytes, err = errorResponse.Marshal()
//
//    platformTokenGetResponse, err := UnmarshalPlatformTokenGetResponse(bytes)
//    bytes, err = platformTokenGetResponse.Marshal()
//
//    platformTokenPutResponse, err := UnmarshalPlatformTokenPutResponse(bytes)
//    bytes, err = platformTokenPutResponse.Marshal()

package satounki

import "encoding/json"

type CompaniesGetResponse []Company

func UnmarshalCompaniesGetResponse(data []byte) (CompaniesGetResponse, error) {
	var r CompaniesGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompaniesGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCompanyGetResponse(data []byte) (CompanyGetResponse, error) {
	var r CompanyGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompanyGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCompanyPostRequest(data []byte) (CompanyPostRequest, error) {
	var r CompanyPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompanyPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCompanyPostResponse(data []byte) (CompanyPostResponse, error) {
	var r CompanyPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompanyPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCompanyPutRequest(data []byte) (CompanyPutRequest, error) {
	var r CompanyPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompanyPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCompanyPutResponse(data []byte) (CompanyPutResponse, error) {
	var r CompanyPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *CompanyPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalErrorResponse(data []byte) (ErrorResponse, error) {
	var r ErrorResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ErrorResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPlatformTokenGetResponse(data []byte) (PlatformTokenGetResponse, error) {
	var r PlatformTokenGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PlatformTokenGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPlatformTokenPutResponse(data []byte) (PlatformTokenPutResponse, error) {
	var r PlatformTokenPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PlatformTokenPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

// Company
type Company struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Company
type CompanyGetResponse struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Company
type CompanyPostRequest struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Company
type CompanyPostResponse struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Company
type CompanyPutRequest struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Company
type CompanyPutResponse struct {
	// Email domain of the company (G-Suite etc.)
	Domain string `json:"domain"`
	// Auto-incrementing integer
	ID int64 `json:"id"`
	// Name of the company
	Name string `json:"name"`
	// Company root user's email address
	RootUserEmail string `json:"root_user_email"`
	// Company root user's first name
	RootUserFirstName *string `json:"root_user_first_name,omitempty"`
	// Company root user's last name
	RootUserLastName *string `json:"root_user_last_name,omitempty"`
}

// Error returned by the Satounki API
type ErrorResponse struct {
	// HTTP error code
	Code int64 `json:"code"`
	// User-friendly error message
	Error string `json:"error"`
}

// Platform API token used for automation
type PlatformTokenGetResponse struct {
	// Token
	Token string `json:"token"`
}

// Platform API token used for automation
type PlatformTokenPutResponse struct {
	// Token
	Token string `json:"token"`
}
