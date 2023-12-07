// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    errorResponse, err := UnmarshalErrorResponse(bytes)
//    bytes, err = errorResponse.Marshal()
//
//    policiesGetResponse, err := UnmarshalPoliciesGetResponse(bytes)
//    bytes, err = policiesGetResponse.Marshal()
//
//    policyGetResponse, err := UnmarshalPolicyGetResponse(bytes)
//    bytes, err = policyGetResponse.Marshal()
//
//    policyNameGetResponse, err := UnmarshalPolicyNameGetResponse(bytes)
//    bytes, err = policyNameGetResponse.Marshal()
//
//    policyPostRequest, err := UnmarshalPolicyPostRequest(bytes)
//    bytes, err = policyPostRequest.Marshal()
//
//    policyPostResponse, err := UnmarshalPolicyPostResponse(bytes)
//    bytes, err = policyPostResponse.Marshal()
//
//    policyPutRequest, err := UnmarshalPolicyPutRequest(bytes)
//    bytes, err = policyPutRequest.Marshal()
//
//    policyPutResponse, err := UnmarshalPolicyPutResponse(bytes)
//    bytes, err = policyPutResponse.Marshal()
//
//    requestAliasGetResponse, err := UnmarshalRequestAliasGetResponse(bytes)
//    bytes, err = requestAliasGetResponse.Marshal()
//
//    requestAliasPatchRequest, err := UnmarshalRequestAliasPatchRequest(bytes)
//    bytes, err = requestAliasPatchRequest.Marshal()
//
//    requestPolicyPostRequest, err := UnmarshalRequestPolicyPostRequest(bytes)
//    bytes, err = requestPolicyPostRequest.Marshal()
//
//    requestPolicyPostResponse, err := UnmarshalRequestPolicyPostResponse(bytes)
//    bytes, err = requestPolicyPostResponse.Marshal()
//
//    requestsGetQueryParams, err := UnmarshalRequestsGetQueryParams(bytes)
//    bytes, err = requestsGetQueryParams.Marshal()
//
//    requestsGetResponse, err := UnmarshalRequestsGetResponse(bytes)
//    bytes, err = requestsGetResponse.Marshal()
//
//    settingsAwsAccountGetResponse, err := UnmarshalSettingsAwsAccountGetResponse(bytes)
//    bytes, err = settingsAwsAccountGetResponse.Marshal()
//
//    settingsAwsAccountPostRequest, err := UnmarshalSettingsAwsAccountPostRequest(bytes)
//    bytes, err = settingsAwsAccountPostRequest.Marshal()
//
//    settingsAwsAccountPostResponse, err := UnmarshalSettingsAwsAccountPostResponse(bytes)
//    bytes, err = settingsAwsAccountPostResponse.Marshal()
//
//    settingsAwsAccountPutRequest, err := UnmarshalSettingsAwsAccountPutRequest(bytes)
//    bytes, err = settingsAwsAccountPutRequest.Marshal()
//
//    settingsAwsAccountPutResponse, err := UnmarshalSettingsAwsAccountPutResponse(bytes)
//    bytes, err = settingsAwsAccountPutResponse.Marshal()
//
//    settingsAwsAccountsGetResponse, err := UnmarshalSettingsAwsAccountsGetResponse(bytes)
//    bytes, err = settingsAwsAccountsGetResponse.Marshal()
//
//    settingsCloudflareAccountGetResponse, err := UnmarshalSettingsCloudflareAccountGetResponse(bytes)
//    bytes, err = settingsCloudflareAccountGetResponse.Marshal()
//
//    settingsCloudflareAccountPostRequest, err := UnmarshalSettingsCloudflareAccountPostRequest(bytes)
//    bytes, err = settingsCloudflareAccountPostRequest.Marshal()
//
//    settingsCloudflareAccountPostResponse, err := UnmarshalSettingsCloudflareAccountPostResponse(bytes)
//    bytes, err = settingsCloudflareAccountPostResponse.Marshal()
//
//    settingsCloudflareAccountPutRequest, err := UnmarshalSettingsCloudflareAccountPutRequest(bytes)
//    bytes, err = settingsCloudflareAccountPutRequest.Marshal()
//
//    settingsCloudflareAccountPutResponse, err := UnmarshalSettingsCloudflareAccountPutResponse(bytes)
//    bytes, err = settingsCloudflareAccountPutResponse.Marshal()
//
//    settingsCloudflareAccountsGetResponse, err := UnmarshalSettingsCloudflareAccountsGetResponse(bytes)
//    bytes, err = settingsCloudflareAccountsGetResponse.Marshal()
//
//    settingsGcpProjectGetResponse, err := UnmarshalSettingsGcpProjectGetResponse(bytes)
//    bytes, err = settingsGcpProjectGetResponse.Marshal()
//
//    settingsGcpProjectPostRequest, err := UnmarshalSettingsGcpProjectPostRequest(bytes)
//    bytes, err = settingsGcpProjectPostRequest.Marshal()
//
//    settingsGcpProjectPostResponse, err := UnmarshalSettingsGcpProjectPostResponse(bytes)
//    bytes, err = settingsGcpProjectPostResponse.Marshal()
//
//    settingsGcpProjectPutRequest, err := UnmarshalSettingsGcpProjectPutRequest(bytes)
//    bytes, err = settingsGcpProjectPutRequest.Marshal()
//
//    settingsGcpProjectPutResponse, err := UnmarshalSettingsGcpProjectPutResponse(bytes)
//    bytes, err = settingsGcpProjectPutResponse.Marshal()
//
//    settingsGcpProjectsGetResponse, err := UnmarshalSettingsGcpProjectsGetResponse(bytes)
//    bytes, err = settingsGcpProjectsGetResponse.Marshal()
//
//    userAliasesGetResponse, err := UnmarshalUserAliasesGetResponse(bytes)
//    bytes, err = userAliasesGetResponse.Marshal()
//
//    userAliasesPostRequest, err := UnmarshalUserAliasesPostRequest(bytes)
//    bytes, err = userAliasesPostRequest.Marshal()
//
//    userAliasesPostResponse, err := UnmarshalUserAliasesPostResponse(bytes)
//    bytes, err = userAliasesPostResponse.Marshal()
//
//    userAliasesPutRequest, err := UnmarshalUserAliasesPutRequest(bytes)
//    bytes, err = userAliasesPutRequest.Marshal()
//
//    userAliasesPutResponse, err := UnmarshalUserAliasesPutResponse(bytes)
//    bytes, err = userAliasesPutResponse.Marshal()
//
//    userRolesGetResponse, err := UnmarshalUserRolesGetResponse(bytes)
//    bytes, err = userRolesGetResponse.Marshal()
//
//    userRolesPostRequest, err := UnmarshalUserRolesPostRequest(bytes)
//    bytes, err = userRolesPostRequest.Marshal()
//
//    userRolesPostResponse, err := UnmarshalUserRolesPostResponse(bytes)
//    bytes, err = userRolesPostResponse.Marshal()
//
//    userRolesPutRequest, err := UnmarshalUserRolesPutRequest(bytes)
//    bytes, err = userRolesPutRequest.Marshal()
//
//    userRolesPutResponse, err := UnmarshalUserRolesPutResponse(bytes)
//    bytes, err = userRolesPutResponse.Marshal()
//
//    userTokenGetResponse, err := UnmarshalUserTokenGetResponse(bytes)
//    bytes, err = userTokenGetResponse.Marshal()
//
//    userTokenPutResponse, err := UnmarshalUserTokenPutResponse(bytes)
//    bytes, err = userTokenPutResponse.Marshal()

package satounki

import "bytes"
import "errors"
import "encoding/json"

func UnmarshalErrorResponse(data []byte) (ErrorResponse, error) {
	var r ErrorResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ErrorResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type PoliciesGetResponse []Policy

func UnmarshalPoliciesGetResponse(data []byte) (PoliciesGetResponse, error) {
	var r PoliciesGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PoliciesGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyGetResponse(data []byte) (PolicyGetResponse, error) {
	var r PolicyGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyNameGetResponse(data []byte) (PolicyNameGetResponse, error) {
	var r PolicyNameGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyNameGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyPostRequest(data []byte) (PolicyPostRequest, error) {
	var r PolicyPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyPostResponse(data []byte) (PolicyPostResponse, error) {
	var r PolicyPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyPutRequest(data []byte) (PolicyPutRequest, error) {
	var r PolicyPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalPolicyPutResponse(data []byte) (PolicyPutResponse, error) {
	var r PolicyPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *PolicyPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalRequestAliasGetResponse(data []byte) (RequestAliasGetResponse, error) {
	var r RequestAliasGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestAliasGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalRequestAliasPatchRequest(data []byte) (RequestAliasPatchRequest, error) {
	var r RequestAliasPatchRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestAliasPatchRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalRequestPolicyPostRequest(data []byte) (RequestPolicyPostRequest, error) {
	var r RequestPolicyPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestPolicyPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalRequestPolicyPostResponse(data []byte) (RequestPolicyPostResponse, error) {
	var r RequestPolicyPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestPolicyPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalRequestsGetQueryParams(data []byte) (RequestsGetQueryParams, error) {
	var r RequestsGetQueryParams
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestsGetQueryParams) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type RequestsGetResponse []Request

func UnmarshalRequestsGetResponse(data []byte) (RequestsGetResponse, error) {
	var r RequestsGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *RequestsGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsAwsAccountGetResponse(data []byte) (SettingsAwsAccountGetResponse, error) {
	var r SettingsAwsAccountGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsAwsAccountPostRequest(data []byte) (SettingsAwsAccountPostRequest, error) {
	var r SettingsAwsAccountPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsAwsAccountPostResponse(data []byte) (SettingsAwsAccountPostResponse, error) {
	var r SettingsAwsAccountPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsAwsAccountPutRequest(data []byte) (SettingsAwsAccountPutRequest, error) {
	var r SettingsAwsAccountPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsAwsAccountPutResponse(data []byte) (SettingsAwsAccountPutResponse, error) {
	var r SettingsAwsAccountPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type SettingsAwsAccountsGetResponse []AwsAccount

func UnmarshalSettingsAwsAccountsGetResponse(data []byte) (SettingsAwsAccountsGetResponse, error) {
	var r SettingsAwsAccountsGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsAwsAccountsGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsCloudflareAccountGetResponse(data []byte) (SettingsCloudflareAccountGetResponse, error) {
	var r SettingsCloudflareAccountGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsCloudflareAccountPostRequest(data []byte) (SettingsCloudflareAccountPostRequest, error) {
	var r SettingsCloudflareAccountPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsCloudflareAccountPostResponse(data []byte) (SettingsCloudflareAccountPostResponse, error) {
	var r SettingsCloudflareAccountPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsCloudflareAccountPutRequest(data []byte) (SettingsCloudflareAccountPutRequest, error) {
	var r SettingsCloudflareAccountPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsCloudflareAccountPutResponse(data []byte) (SettingsCloudflareAccountPutResponse, error) {
	var r SettingsCloudflareAccountPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type SettingsCloudflareAccountsGetResponse []CloudflareAccount

func UnmarshalSettingsCloudflareAccountsGetResponse(data []byte) (SettingsCloudflareAccountsGetResponse, error) {
	var r SettingsCloudflareAccountsGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsCloudflareAccountsGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsGcpProjectGetResponse(data []byte) (SettingsGcpProjectGetResponse, error) {
	var r SettingsGcpProjectGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsGcpProjectPostRequest(data []byte) (SettingsGcpProjectPostRequest, error) {
	var r SettingsGcpProjectPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsGcpProjectPostResponse(data []byte) (SettingsGcpProjectPostResponse, error) {
	var r SettingsGcpProjectPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsGcpProjectPutRequest(data []byte) (SettingsGcpProjectPutRequest, error) {
	var r SettingsGcpProjectPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalSettingsGcpProjectPutResponse(data []byte) (SettingsGcpProjectPutResponse, error) {
	var r SettingsGcpProjectPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type SettingsGcpProjectsGetResponse []GcpProject

func UnmarshalSettingsGcpProjectsGetResponse(data []byte) (SettingsGcpProjectsGetResponse, error) {
	var r SettingsGcpProjectsGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *SettingsGcpProjectsGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserAliasesGetResponse(data []byte) (UserAliasesGetResponse, error) {
	var r UserAliasesGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserAliasesGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserAliasesPostRequest(data []byte) (UserAliasesPostRequest, error) {
	var r UserAliasesPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserAliasesPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserAliasesPostResponse(data []byte) (UserAliasesPostResponse, error) {
	var r UserAliasesPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserAliasesPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserAliasesPutRequest(data []byte) (UserAliasesPutRequest, error) {
	var r UserAliasesPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserAliasesPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserAliasesPutResponse(data []byte) (UserAliasesPutResponse, error) {
	var r UserAliasesPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserAliasesPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserRolesGetResponse []AccessRole

func UnmarshalUserRolesGetResponse(data []byte) (UserRolesGetResponse, error) {
	var r UserRolesGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserRolesGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserRolesPostRequest []AccessRole

func UnmarshalUserRolesPostRequest(data []byte) (UserRolesPostRequest, error) {
	var r UserRolesPostRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserRolesPostRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserRolesPostResponse []AccessRole

func UnmarshalUserRolesPostResponse(data []byte) (UserRolesPostResponse, error) {
	var r UserRolesPostResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserRolesPostResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserRolesPutRequest []AccessRole

func UnmarshalUserRolesPutRequest(data []byte) (UserRolesPutRequest, error) {
	var r UserRolesPutRequest
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserRolesPutRequest) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type UserRolesPutResponse []AccessRole

func UnmarshalUserRolesPutResponse(data []byte) (UserRolesPutResponse, error) {
	var r UserRolesPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserRolesPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserTokenGetResponse(data []byte) (UserTokenGetResponse, error) {
	var r UserTokenGetResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserTokenGetResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalUserTokenPutResponse(data []byte) (UserTokenPutResponse, error) {
	var r UserTokenPutResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *UserTokenPutResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

// Error returned by the Satounki API
type ErrorResponse struct {
	// HTTP error code
	Code int64 `json:"code"`
	// User-friendly error message
	Error string `json:"error"`
}

// Satounki Policy definition
type Policy struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyGetResponse struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyNameGetResponse struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyPostRequest struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyPostResponse struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyPutRequest struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Satounki Policy definition
type PolicyPutResponse struct {
	// Amazon Web Services policy ARNs associated with this policy
	Aws []string `json:"aws,omitempty"`
	// Cloudflare roles associated with this policy
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Description of the permissions granted by this policy
	Description string `json:"description"`
	// Google Cloud Platform roles associated with this policy
	Gcp []string `json:"gcp,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Succinct, descriptive name for the policy in snake_case
	Name string `json:"name"`
}

// Access request
type RequestAliasGetResponse struct {
	// Access request permissions expiry timestamp
	AccessExpiry *string `json:"access_expiry,omitempty"`
	// Administrator approval requirement
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Human-friendly alias generated by Satounki
	Alias string `json:"alias"`
	// Request approval records
	Approvals []RequestAliasGetResponseUserInteraction `json:"approvals,omitempty"`
	// Number of approvals required
	ApprovalsRequired int64 `json:"approvals_required"`
	// Approval status
	Approved bool `json:"approved"`
	// Amazon Web Services policy ARNs requested
	Aws []string `json:"aws,omitempty"`
	// Amazon Web Services account to grant permissions on
	AwsAccount *string `json:"aws_account,omitempty"`
	// Request cancellation record
	Cancellation *RequestAliasGetResponseUserInteraction `json:"cancellation,omitempty"`
	// Cloudflare roles requested
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Cloudflare account to grant permissions on
	CloudflareAccount *string `json:"cloudflare_account,omitempty"`
	// Request extension records
	Extensions []RequestAliasGetResponseUserInteraction `json:"extensions,omitempty"`
	// Google Cloud Platform roles requested
	Gcp []string `json:"gcp,omitempty"`
	// Google Cloud Platform project to grant permissions on
	GcpProject *string `json:"gcp_project,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Reason for the request
	Justification string `json:"justification"`
	// Duration of the request in minutes
	Minutes int64 `json:"minutes"`
	// Policy name
	Policy string `json:"policy"`
	// Request rejection record
	Rejection *RequestAliasGetResponseUserInteraction `json:"rejection,omitempty"`
	// Email address of the requester
	Requester string `json:"requester"`
	// Service-specific username aliases of the requester
	RequesterAliases RequestAliasGetResponseRequesterAliases `json:"requester_aliases"`
	// Current state in the access request lifecycle
	State AccessRequestState `json:"state"`
	// Access request timestamp
	Timestamp string `json:"timestamp"`
}

// Record of a user interaction with an access request
type RequestAliasGetResponseUserInteraction struct {
	// User interaction timestamp
	Timestamp string `json:"timestamp"`
	// Interacting user's email address
	User string `json:"user"`
}

// Service-specific username aliases of the requester
//
// Service-specific username aliases
type RequestAliasGetResponseRequesterAliases struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Extend an active request by N minutes
type RequestAliasPatchRequestClass struct {
	Extend int64 `json:"extend"`
}

// Access request for policy permissions
type RequestPolicyPostRequest struct {
	// AWS account to grant permissions on, if the policy includes AWS policy ARNs
	AwsAccount *string `json:"aws_account,omitempty"`
	// Cloudflare account to grant permissions on, if the policy includes Cloudflare roles
	CloudflareAccount *string `json:"cloudflare_account,omitempty"`
	// GCP project to grant permissions on, if the policy includes GCP roles
	GcpProject *string `json:"gcp_project,omitempty"`
	// Reason for the request
	Justification string `json:"justification"`
	// Duration of the request in minutes
	Minutes int64 `json:"minutes"`
}

// Access request confirmation
type RequestPolicyPostResponse struct {
	// Human-friendly alias generated by Satounki
	RequestAlias string `json:"request_alias"`
	// UUID generated by Satounki
	RequestID string `json:"request_id"`
}

// Query parameters for the GET /v1/requests endpoint
type RequestsGetQueryParams struct {
	// Number of requests to return
	Count int64 `json:"count"`
	// State of the requests
	State AccessRequestState `json:"state"`
}

// Access request
type Request struct {
	// Access request permissions expiry timestamp
	AccessExpiry *string `json:"access_expiry,omitempty"`
	// Administrator approval requirement
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Human-friendly alias generated by Satounki
	Alias string `json:"alias"`
	// Request approval records
	Approvals []RequestsGetResponseUserInteraction `json:"approvals,omitempty"`
	// Number of approvals required
	ApprovalsRequired int64 `json:"approvals_required"`
	// Approval status
	Approved bool `json:"approved"`
	// Amazon Web Services policy ARNs requested
	Aws []string `json:"aws,omitempty"`
	// Amazon Web Services account to grant permissions on
	AwsAccount *string `json:"aws_account,omitempty"`
	// Request cancellation record
	Cancellation *RequestsGetResponseUserInteraction `json:"cancellation,omitempty"`
	// Cloudflare roles requested
	Cloudflare []CloudflareRole `json:"cloudflare,omitempty"`
	// Cloudflare account to grant permissions on
	CloudflareAccount *string `json:"cloudflare_account,omitempty"`
	// Request extension records
	Extensions []RequestsGetResponseUserInteraction `json:"extensions,omitempty"`
	// Google Cloud Platform roles requested
	Gcp []string `json:"gcp,omitempty"`
	// Google Cloud Platform project to grant permissions on
	GcpProject *string `json:"gcp_project,omitempty"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Reason for the request
	Justification string `json:"justification"`
	// Duration of the request in minutes
	Minutes int64 `json:"minutes"`
	// Policy name
	Policy string `json:"policy"`
	// Request rejection record
	Rejection *RequestsGetResponseUserInteraction `json:"rejection,omitempty"`
	// Email address of the requester
	Requester string `json:"requester"`
	// Service-specific username aliases of the requester
	RequesterAliases RequestsGetResponseRequesterAliases `json:"requester_aliases"`
	// Current state in the access request lifecycle
	State AccessRequestState `json:"state"`
	// Access request timestamp
	Timestamp string `json:"timestamp"`
}

// Record of a user interaction with an access request
type RequestsGetResponseUserInteraction struct {
	// User interaction timestamp
	Timestamp string `json:"timestamp"`
	// Interacting user's email address
	User string `json:"user"`
}

// Service-specific username aliases of the requester
//
// Service-specific username aliases
type RequestsGetResponseRequesterAliases struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Amazon Web Services account configuration
type SettingsAwsAccountGetResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Amazon Web Services account configuration
type SettingsAwsAccountPostRequest struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
}

// Amazon Web Services account configuration
type SettingsAwsAccountPostResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Amazon Web Services account configuration
type SettingsAwsAccountPutRequest struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
}

// Amazon Web Services account configuration
type SettingsAwsAccountPutResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Amazon Web Services account configuration
type AwsAccount struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Cloudflare account configuration
type SettingsCloudflareAccountGetResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Cloudflare account configuration
type SettingsCloudflareAccountPostRequest struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
}

// Cloudflare account configuration
type SettingsCloudflareAccountPostResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Cloudflare account configuration
type SettingsCloudflareAccountPutRequest struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
}

// Cloudflare account configuration
type SettingsCloudflareAccountPutResponse struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Cloudflare account configuration
type CloudflareAccount struct {
	// Meaningful alias for the account to be used by Satounki users
	Account string `json:"account"`
	// Require additional approval by an Administrator for access requests made to the account
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the account
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
}

// Google Cloud Platform project configuration
type SettingsGcpProjectGetResponse struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Google Cloud Platform project configuration
type SettingsGcpProjectPostRequest struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Google Cloud Platform project configuration
type SettingsGcpProjectPostResponse struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Google Cloud Platform project configuration
type SettingsGcpProjectPutRequest struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Google Cloud Platform project configuration
type SettingsGcpProjectPutResponse struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Google Cloud Platform project configuration
type GcpProject struct {
	// Require additional approval by an Administrator for access requests made to the project
	AdminApprovalRequired bool `json:"admin_approval_required"`
	// Number of approvals required for access requests made to the project
	ApprovalsRequired int64 `json:"approvals_required"`
	// UUID generated by Satounki
	ID string `json:"id"`
	// Meaningful alias for the project to be used by Satounki users
	Project string `json:"project"`
}

// Service-specific username aliases
type UserAliasesGetResponse struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Service-specific username aliases
type UserAliasesPostRequest struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Service-specific username aliases
type UserAliasesPostResponse struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Service-specific username aliases
type UserAliasesPutRequest struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// Service-specific username aliases
type UserAliasesPutResponse struct {
	// Username on Amazon Web Services, may not be an email address
	Aws *string `json:"aws,omitempty"`
	// Email address registered with Cloudflare
	Cloudflare *string `json:"cloudflare,omitempty"`
	// Email address registered with Google Cloud Platform
	Gcp *string `json:"gcp,omitempty"`
}

// User API token for personal use
type UserTokenGetResponse struct {
	// Token
	Token string `json:"token"`
}

// User API token for personal use
type UserTokenPutResponse struct {
	// Token
	Token string `json:"token"`
}

// Cloudflare role
type CloudflareRole string

const (
	Administrator                CloudflareRole = "Administrator"
	AdministratorReadOnly        CloudflareRole = "AdministratorReadOnly"
	Analytics                    CloudflareRole = "Analytics"
	AuditLogsViewer              CloudflareRole = "AuditLogsViewer"
	Billing                      CloudflareRole = "Billing"
	CachePurge                   CloudflareRole = "CachePurge"
	CloudflareAccess             CloudflareRole = "CloudflareAccess"
	CloudflareGateway            CloudflareRole = "CloudflareGateway"
	CloudflareImages             CloudflareRole = "CloudflareImages"
	CloudflareStream             CloudflareRole = "CloudflareStream"
	CloudflareWorkersAdmin       CloudflareRole = "CloudflareWorkersAdmin"
	CloudflareZeroTrust          CloudflareRole = "CloudflareZeroTrust"
	CloudflareZeroTrustPii       CloudflareRole = "CloudflareZeroTrustPii"
	CloudflareZeroTrustReadOnly  CloudflareRole = "CloudflareZeroTrustReadOnly"
	CloudflareZeroTrustReporting CloudflareRole = "CloudflareZeroTrustReporting"
	DNS                          CloudflareRole = "Dns"
	Firewall                     CloudflareRole = "Firewall"
	LoadBalancer                 CloudflareRole = "LoadBalancer"
	LogShare                     CloudflareRole = "LogShare"
	LogShareReader               CloudflareRole = "LogShareReader"
)

// Current state in the access request lifecycle
//
// # State in the access request lifecycle
//
// # Request has been submitted and may or may not have met required approvals
//
// Request has been approved and the permissions associated with the policy have been
// granted
//
// # Request has expired or been marked as completed early by the requesting user
//
// # Request has been cancelled before approval by the requesting user
//
// # Request has been rejected by an Approver or an Administrator
//
// # Request was active, but revoked by an Administrator
//
// State of the requests
type AccessRequestState string

const (
	Active    AccessRequestState = "active"
	Cancelled AccessRequestState = "cancelled"
	Completed AccessRequestState = "completed"
	Pending   AccessRequestState = "pending"
	Rejected  AccessRequestState = "rejected"
	Revoked   AccessRequestState = "revoked"
)

// Add an approval to a pending request
//
// # Reject a pending request
//
// # Cancel a pending request
//
// # Complete an active request
//
// Revoke permissions from an active request
type RequestAliasPatchRequestEnum string

const (
	Approve  RequestAliasPatchRequestEnum = "approve"
	Cancel   RequestAliasPatchRequestEnum = "cancel"
	Complete RequestAliasPatchRequestEnum = "complete"
	Reject   RequestAliasPatchRequestEnum = "reject"
	Revoke   RequestAliasPatchRequestEnum = "revoke"
)

// Satounki user access roles
//
// # View and make access requests
//
// # Approve access requests
//
// Change user roles, grant administrator approval to access requests
type AccessRole string

const (
	AccessRoleAdministrator AccessRole = "administrator"
	Approver                AccessRole = "approver"
	User                    AccessRole = "user"
)

// Operation on an access request
type RequestAliasPatchRequest struct {
	Enum                          *RequestAliasPatchRequestEnum
	RequestAliasPatchRequestClass *RequestAliasPatchRequestClass
}

func (x *RequestAliasPatchRequest) UnmarshalJSON(data []byte) error {
	x.RequestAliasPatchRequestClass = nil
	x.Enum = nil
	var c RequestAliasPatchRequestClass
	object, err := unmarshalUnion(data, nil, nil, nil, nil, false, nil, true, &c, false, nil, true, &x.Enum, false)
	if err != nil {
		return err
	}
	if object {
		x.RequestAliasPatchRequestClass = &c
	}
	return nil
}

func (x *RequestAliasPatchRequest) MarshalJSON() ([]byte, error) {
	return marshalUnion(nil, nil, nil, nil, false, nil, x.RequestAliasPatchRequestClass != nil, x.RequestAliasPatchRequestClass, false, nil, x.Enum != nil, x.Enum, false)
}

func unmarshalUnion(data []byte, pi **int64, pf **float64, pb **bool, ps **string, haveArray bool, pa interface{}, haveObject bool, pc interface{}, haveMap bool, pm interface{}, haveEnum bool, pe interface{}, nullable bool) (bool, error) {
	if pi != nil {
		*pi = nil
	}
	if pf != nil {
		*pf = nil
	}
	if pb != nil {
		*pb = nil
	}
	if ps != nil {
		*ps = nil
	}

	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	tok, err := dec.Token()
	if err != nil {
		return false, err
	}

	switch v := tok.(type) {
	case json.Number:
		if pi != nil {
			i, err := v.Int64()
			if err == nil {
				*pi = &i
				return false, nil
			}
		}
		if pf != nil {
			f, err := v.Float64()
			if err == nil {
				*pf = &f
				return false, nil
			}
			return false, errors.New("Unparsable number")
		}
		return false, errors.New("Union does not contain number")
	case float64:
		return false, errors.New("Decoder should not return float64")
	case bool:
		if pb != nil {
			*pb = &v
			return false, nil
		}
		return false, errors.New("Union does not contain bool")
	case string:
		if haveEnum {
			return false, json.Unmarshal(data, pe)
		}
		if ps != nil {
			*ps = &v
			return false, nil
		}
		return false, errors.New("Union does not contain string")
	case nil:
		if nullable {
			return false, nil
		}
		return false, errors.New("Union does not contain null")
	case json.Delim:
		if v == '{' {
			if haveObject {
				return true, json.Unmarshal(data, pc)
			}
			if haveMap {
				return false, json.Unmarshal(data, pm)
			}
			return false, errors.New("Union does not contain object")
		}
		if v == '[' {
			if haveArray {
				return false, json.Unmarshal(data, pa)
			}
			return false, errors.New("Union does not contain array")
		}
		return false, errors.New("Cannot handle delimiter")
	}
	return false, errors.New("Cannot unmarshal union")

}

func marshalUnion(pi *int64, pf *float64, pb *bool, ps *string, haveArray bool, pa interface{}, haveObject bool, pc interface{}, haveMap bool, pm interface{}, haveEnum bool, pe interface{}, nullable bool) ([]byte, error) {
	if pi != nil {
		return json.Marshal(*pi)
	}
	if pf != nil {
		return json.Marshal(*pf)
	}
	if pb != nil {
		return json.Marshal(*pb)
	}
	if ps != nil {
		return json.Marshal(*ps)
	}
	if haveArray {
		return json.Marshal(pa)
	}
	if haveObject {
		return json.Marshal(pc)
	}
	if haveMap {
		return json.Marshal(pm)
	}
	if haveEnum {
		return json.Marshal(pe)
	}
	if nullable {
		return json.Marshal(nil)
	}
	return nil, errors.New("Union must not be null")
}
