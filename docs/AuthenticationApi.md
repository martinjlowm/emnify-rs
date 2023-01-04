# \AuthenticationApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate**](AuthenticationApi.md#authenticate) | **POST** /api/v1/authenticate | Retrieve Authentication Token
[**post_mfa**](AuthenticationApi.md#post_mfa) | **POST** /api/v1/user/mfa | Create an MFA key
[**user_mfa_by_id_patch**](AuthenticationApi.md#user_mfa_by_id_patch) | **PATCH** /api/v1/user/mfa/{key_id} | Activate MFA key
[**user_mfa_by_user_id_and_key_id_delete**](AuthenticationApi.md#user_mfa_by_user_id_and_key_id_delete) | **DELETE** /api/v1/user/{user_id}/mfa/{key_id} | Delete an MFA key
[**user_mfa_status_get**](AuthenticationApi.md#user_mfa_status_get) | **GET** /api/v1/user/mfa/status | List MFA key Statuses
[**user_mfa_trusted_device_by_user_id_and_device_id_delete**](AuthenticationApi.md#user_mfa_trusted_device_by_user_id_and_device_id_delete) | **DELETE** /api/v1/user/{user_id}/mfa/trusted_device/{device_id} | Delete a Trusted Device
[**user_mfa_trusted_device_by_user_id_get**](AuthenticationApi.md#user_mfa_trusted_device_by_user_id_get) | **GET** /api/v1/user/{user_id}/mfa/trusted_device | List Trusted Devices
[**user_mfa_type_get**](AuthenticationApi.md#user_mfa_type_get) | **GET** /api/v1/user/mfa/type | List MFA key types



## authenticate

> crate::models::AuthenticationResponse authenticate(authentication)
Retrieve Authentication Token

This entrypoint returns a JWT `auth_token` for authenticating further requests to the API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authentication** | [**Authentication**](Authentication.md) | Authentication using Application Tokens or user/password combination | [required] |

### Return type

[**crate::models::AuthenticationResponse**](AuthenticationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_mfa

> crate::models::CreateMfaKeyResponse post_mfa(post_mfa_request)
Create an MFA key

Generate and store a MFA key for the requesting user. The MFA key will have the status `activation pending` after this call and must be activated through a separate call (`/api/v1/user/mfa/{id}`). You must provide following fields with this request:  * `type` (Object required)   - id (Number) * `password` (String required) - User password   The **MFA key** object returned by the server contains the following properties:  * `id` (Integer) - The unique ID of this MFA key * `status` (Object) ID (Integer) - Id of status of this MFA key   - `description` (String) - description of the status * `type` (Object) ID (Integer) - Id of type of this MFA key   - `description` (String) - description of the type * `secret_key` (String) - Secret key (encoded in Base32) for this MFA key, will be displayed only on creation * `otpauth` (String) - Secret key as a URI encoded for QR codes, will be displayed only on creation * `creation_date` (Timestamp) - Timestamp when this MFA key was created - type: ISO 8601 timestamp format * `activation_date` (Timestamp) - Timestamp when this MFA key was activated - type: ISO 8601 timestamp format 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_mfa_request** | [**PostMfaRequest**](PostMfaRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMfaKeyResponse**](Create_MFA_Key_Response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_by_id_patch

> user_mfa_by_id_patch(key_id, activate_mfa_key_request)
Activate MFA key

Activate the MFA key of the requesting user.  You must provide following JSON fields in this request:  * `status` (Object required)   - `id` (Number) use 1 for \"ACTIVE\" status  * `code` (String required)  - the 6-digit \"time-based one-time password\" (TOTP) generated with this MFA key for the current Time-Step 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **f32** | Key ID | [required] |
**activate_mfa_key_request** | [**ActivateMfaKeyRequest**](ActivateMfaKeyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_by_user_id_and_key_id_delete

> user_mfa_by_user_id_and_key_id_delete(key_id, user_id)
Delete an MFA key

Delete an MFA key for a given user.  An own MFA key can also be deleted with a call to `/api/v1/user/my/mfa/{key_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **f32** | Key ID | [required] |
**user_id** | **f32** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_status_get

> Vec<crate::models::MfaKeyStatusLookupresponse> user_mfa_status_get()
List MFA key Statuses

Retrieve a list of possible MFA Key statuses.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MfaKeyStatusLookupresponse>**](MFAKeyStatusLookupresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_trusted_device_by_user_id_and_device_id_delete

> user_mfa_trusted_device_by_user_id_and_device_id_delete(user_id, device_id)
Delete a Trusted Device

Deletes a trusted device.  Removing one's own trusted device can also be performed at either `/api/v1/user/my/mfa/trusted_device/{id}` or `/api/v1/user/mfa/trusted_device/{id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**device_id** | **f32** | Device ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_trusted_device_by_user_id_get

> Vec<crate::models::Listoftrusteddevicesresponse> user_mfa_trusted_device_by_user_id_get(user_id)
List Trusted Devices

Returns the list of trusted devices for a given user.  The list of one's own trusted devices can also be retrieved with a call to either `/api/v1/user/my/mfa/trusted_device` or `/api/v1/user/mfa/trusted_device` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |

### Return type

[**Vec<crate::models::Listoftrusteddevicesresponse>**](Listoftrusteddevicesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mfa_type_get

> Vec<crate::models::MfaKeyTypeLookupresponse> user_mfa_type_get()
List MFA key types

Retrieve a list of possible MFA Key types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MfaKeyTypeLookupresponse>**](MFAKeyTypeLookupresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

