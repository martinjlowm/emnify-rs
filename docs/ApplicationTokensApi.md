# \ApplicationTokensApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**application_token_by_id_patch**](ApplicationTokensApi.md#application_token_by_id_patch) | **PATCH** /api/v1/application_token/{app_token_id} | Update Application Token
[**application_token_get**](ApplicationTokensApi.md#application_token_get) | **GET** /api/v1/application_token | List Application Tokens
[**application_token_post**](ApplicationTokensApi.md#application_token_post) | **POST** /api/v1/application_token | Create Application Token



## application_token_by_id_patch

> application_token_by_id_patch(app_token_id, application_token)
Update Application Token

The `description` of the token may be updated and the Application Token can be revoked by updating the `status`. The possible statuses of tokens are  * `\"id\": 0` - Activated * `\"id\": 1` - Revoked 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_token_id** | **f32** | application token ID | [required] |
**application_token** | [**ApplicationToken**](ApplicationToken.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_token_get

> Vec<crate::models::ListofApplicationTokensresponse> application_token_get()
List Application Tokens

Returns the list of application tokens for the organisation of the requesting user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ListofApplicationTokensresponse>**](ListofApplicationTokensresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_token_post

> crate::models::CreateApplicationTokenresponse application_token_post(create_application_tokenrequest)
Create Application Token

Creates a new application token. ID must not be specified, as it is auto-generated and returned in case of a successful JSON response. You can provide following fields with this request:  * `description` (String, optional) * `expiry_date` with optional time + time zone (String, optional) * `ip` - IP Address in CIDR notation (String, optional) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_application_tokenrequest** | [**CreateApplicationTokenrequest**](CreateApplicationTokenrequest.md) |  | [required] |

### Return type

[**crate::models::CreateApplicationTokenresponse**](CreateApplicationTokenresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

