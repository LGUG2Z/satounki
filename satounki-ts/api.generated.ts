// Generated by satounki/common-gen

import { IRestResponse, RestClient } from "typed-rest-client/RestClient";
import * as satounki from "./types.generated";

export class Api {
  private apiToken: string;
  private baseUrl: string;
  private userAgent: string;
  client: RestClient;

  constructor(apiToken: string, baseUrl: string, userAgent: string) {
    let restClient = new RestClient(userAgent, baseUrl, [], {
      headers: {
        Authorization: `Bearer ${apiToken}`,
      },
    });

    this.apiToken = apiToken;
    this.baseUrl = baseUrl;
    this.userAgent = userAgent;
    this.client = restClient;
  }

  async policiesGet(): Promise<
    IRestResponse<satounki.PoliciesGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get("/v1/policies");
  }

  async policyNameGet(
    id: string,
  ): Promise<
    IRestResponse<satounki.PolicyNameGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get(`/v1/policy/name/${id}`);
  }

  async policyPost(
    body: satounki.PolicyPostRequest,
  ): Promise<
    IRestResponse<satounki.PolicyPostResponse | satounki.ErrorResponse>
  > {
    return await this.client.create("/v1/policy", body);
  }

  async policyPut(
    id: string,
    body: satounki.PolicyPutRequest,
  ): Promise<
    IRestResponse<satounki.PolicyPutResponse | satounki.ErrorResponse>
  > {
    return await this.client.replace(`/v1/policy/${id}`, body);
  }

  async policyGet(
    id: string,
  ): Promise<
    IRestResponse<satounki.PolicyGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get(`/v1/policy/${id}`);
  }

  async policyDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/policy/${id}`);
  }

  async settingsAwsAccountsGet(): Promise<
    IRestResponse<
      satounki.SettingsAwsAccountsGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get("/v1/settings/aws-accounts");
  }

  async settingsCloudflareAccountsGet(): Promise<
    IRestResponse<
      satounki.SettingsCloudflareAccountsGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get("/v1/settings/cloudflare-accounts");
  }

  async settingsGcpProjectsGet(): Promise<
    IRestResponse<
      satounki.SettingsGcpProjectsGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get("/v1/settings/gcp-projects");
  }

  async settingsAwsAccountPost(
    body: satounki.SettingsAwsAccountPostRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsAwsAccountPostResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.create("/v1/settings/aws-account", body);
  }

  async settingsAwsAccountPut(
    id: string,
    body: satounki.SettingsAwsAccountPutRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsAwsAccountPutResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.replace(`/v1/settings/aws-account/${id}`, body);
  }

  async settingsAwsAccountGet(
    id: string,
  ): Promise<
    IRestResponse<
      satounki.SettingsAwsAccountGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get(`/v1/settings/aws-account/${id}`);
  }

  async settingsAwsAccountDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/settings/aws-account/${id}`);
  }

  async settingsCloudflareAccountPost(
    body: satounki.SettingsCloudflareAccountPostRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsCloudflareAccountPostResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.create("/v1/settings/cloudflare-account", body);
  }

  async settingsCloudflareAccountPut(
    id: string,
    body: satounki.SettingsCloudflareAccountPutRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsCloudflareAccountPutResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.replace(
      `/v1/settings/cloudflare-account/${id}`,
      body,
    );
  }

  async settingsCloudflareAccountGet(
    id: string,
  ): Promise<
    IRestResponse<
      satounki.SettingsCloudflareAccountGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get(`/v1/settings/cloudflare-account/${id}`);
  }

  async settingsCloudflareAccountDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/settings/cloudflare-account/${id}`);
  }

  async settingsGcpProjectPost(
    body: satounki.SettingsGcpProjectPostRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsGcpProjectPostResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.create("/v1/settings/gcp-project", body);
  }

  async settingsGcpProjectPut(
    id: string,
    body: satounki.SettingsGcpProjectPutRequest,
  ): Promise<
    IRestResponse<
      satounki.SettingsGcpProjectPutResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.replace(`/v1/settings/gcp-project/${id}`, body);
  }

  async settingsGcpProjectGet(
    id: string,
  ): Promise<
    IRestResponse<
      satounki.SettingsGcpProjectGetResponse | satounki.ErrorResponse
    >
  > {
    return await this.client.get(`/v1/settings/gcp-project/${id}`);
  }

  async settingsGcpProjectDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/settings/gcp-project/${id}`);
  }

  async userAliasesPost(
    id: string,
    body: satounki.UserAliasesPostRequest,
  ): Promise<
    IRestResponse<satounki.UserAliasesPostResponse | satounki.ErrorResponse>
  > {
    return await this.client.create(`/v1/user/${id}/aliases`, body);
  }

  async userAliasesPut(
    id: string,
    body: satounki.UserAliasesPutRequest,
  ): Promise<
    IRestResponse<satounki.UserAliasesPutResponse | satounki.ErrorResponse>
  > {
    return await this.client.replace(`/v1/user/${id}/aliases`, body);
  }

  async userAliasesGet(
    id: string,
  ): Promise<
    IRestResponse<satounki.UserAliasesGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get(`/v1/user/${id}/aliases`);
  }

  async userAliasesDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/user/${id}/aliases`);
  }

  async userRolesPost(
    id: string,
    body: satounki.UserRolesPostRequest,
  ): Promise<
    IRestResponse<satounki.UserRolesPostResponse | satounki.ErrorResponse>
  > {
    return await this.client.create(`/v1/user/${id}/roles`, body);
  }

  async userRolesPut(
    id: string,
    body: satounki.UserRolesPutRequest,
  ): Promise<
    IRestResponse<satounki.UserRolesPutResponse | satounki.ErrorResponse>
  > {
    return await this.client.replace(`/v1/user/${id}/roles`, body);
  }

  async userRolesGet(
    id: string,
  ): Promise<
    IRestResponse<satounki.UserRolesGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get(`/v1/user/${id}/roles`);
  }

  async userRolesDelete(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.del(`/v1/user/${id}/roles`);
  }

  async userTokenGet(): Promise<
    IRestResponse<satounki.UserTokenGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get("/v1/user/token");
  }

  async userTokenPut(): Promise<
    IRestResponse<satounki.UserTokenPutResponse | satounki.ErrorResponse>
  > {
    return await this.client.replace("/v1/user/token", null);
  }

  async userEnablePatch(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.update(`/v1/user/${id}/enable`, {});
  }

  async userDisablePatch(
    id: string,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.update(`/v1/user/${id}/enable`, {});
  }

  async requestAliasGet(
    id: string,
  ): Promise<
    IRestResponse<satounki.RequestAliasGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get(`/v1/request/alias/${id}`);
  }

  async requestAliasPatch(
    id: string,
    body: satounki.RequestAliasPatchRequestEnum,
  ): Promise<IRestResponse<null | satounki.ErrorResponse>> {
    return await this.client.update(`/v1/request/alias/${id}`, body);
  }

  async requestPolicyPost(
    id: string,
    body: satounki.RequestPolicyPostRequest,
  ): Promise<
    IRestResponse<satounki.RequestPolicyPostResponse | satounki.ErrorResponse>
  > {
    return await this.client.create(`/v1/request/policy/${id}`, body);
  }

  async requestsGet(): Promise<
    IRestResponse<satounki.RequestsGetResponse | satounki.ErrorResponse>
  > {
    return await this.client.get("/v1/requests");
  }
}
