# \IntegrationsApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_callback_secret**](IntegrationsApi.md#create_callback_secret) | **POST** /api/v1/api_secret | Create a Callback Secret
[**create_callback_url**](IntegrationsApi.md#create_callback_url) | **POST** /api/v1/api_callback | Create a Callback URL
[**create_data_streamer**](IntegrationsApi.md#create_data_streamer) | **POST** /api/v2/data_stream | Create Data Stream
[**data_streamer_v2_by_id_restart**](IntegrationsApi.md#data_streamer_v2_by_id_restart) | **POST** /api/v2/data_stream/{data_stream_id}/restart | Restart Existing Data Stream
[**delete_callback_secret**](IntegrationsApi.md#delete_callback_secret) | **DELETE** /api/v1/api_secret/{api_secret_id} | Delete a Callback Secret
[**delete_callback_url**](IntegrationsApi.md#delete_callback_url) | **DELETE** /api/v1/api_callback/{api_callback_id} | Delete a Callback URL
[**delete_data_streamer_v2**](IntegrationsApi.md#delete_data_streamer_v2) | **DELETE** /api/v2/data_stream/{data_stream_id} | Delete Existing Data Stream
[**get_api_callback_secret**](IntegrationsApi.md#get_api_callback_secret) | **GET** /api/v1/api_secret | List API Callback Secrets
[**get_api_callback_urls**](IntegrationsApi.md#get_api_callback_urls) | **GET** /api/v1/api_callback | Retrieve list of API Callback URLs
[**get_callback_secretby_id**](IntegrationsApi.md#get_callback_secretby_id) | **GET** /api/v1/api_secret/{api_secret_id} | Get a Callback Secret by ID
[**get_callback_ur_lby_id**](IntegrationsApi.md#get_callback_ur_lby_id) | **GET** /api/v1/api_callback/{api_callback_id} | Get a Callback URL by ID
[**get_data_streamer_by_id_v2**](IntegrationsApi.md#get_data_streamer_by_id_v2) | **GET** /api/v2/data_stream/{data_stream_id} | Get Details on Existing Data Stream
[**get_data_streamer_data_stream_type**](IntegrationsApi.md#get_data_streamer_data_stream_type) | **GET** /api/v2/data_stream/type | Get Data Stream Types
[**get_data_streamer_enum_filter_field_type**](IntegrationsApi.md#get_data_streamer_enum_filter_field_type) | **GET** /api/v2/data_stream/filter_field_type | Get Possible Data Stream Filter Fields
[**get_data_streamer_statuses**](IntegrationsApi.md#get_data_streamer_statuses) | **GET** /api/v2/data_stream/status | Get Possible Data Stream Statuses
[**list_data_streamer_v2s**](IntegrationsApi.md#list_data_streamer_v2s) | **GET** /api/v2/data_stream | List Data Stream configurations of your organization
[**patch_v2_data_stream**](IntegrationsApi.md#patch_v2_data_stream) | **PATCH** /api/v2/data_stream/{data_stream_id} | Modify Existing Data Stream



## create_callback_secret

> create_callback_secret(get_api_callback_secret200_response_inner)
Create a Callback Secret

Creates a new secret that may be used by API callbacks.  When an `api_secret` is assigned to a Service Profile with an API callback, API requests towards this URL will contain an `Authorization` header with a JSON Web Token. The `api_secret` is used as the __Signing Key__ of the JWT. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_api_callback_secret200_response_inner** | [**GetApiCallbackSecret200ResponseInner**](GetApiCallbackSecret200ResponseInner.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_callback_url

> create_callback_url(body)
Create a Callback URL

Create Callback URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> | Creating an API callback URL | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_data_streamer

> create_data_streamer(list_data_streamer_v2s200_response)
Create Data Stream

Create Data Stream to your destination. Click on Examples to see available Data Streams and configuration parameters.  Currently available `connection_types` are - `AwsKinesis` - `S3` - `RestAPI` - `KeenIO` - `Datadog` - `EventHubs` - `PubSub` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_data_streamer_v2s200_response** | [**ListDataStreamerV2s200Response**](ListDataStreamerV2s200Response.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_streamer_v2_by_id_restart

> data_streamer_v2_by_id_restart(data_stream_id)
Restart Existing Data Stream

Restart your Existing Data Stream. This might be necessary when e.g. your Webhook endpoint has been unreachable for an extended period of time. Webhook Data Stream will exhaust retries and go into Error State. By using this API call your Data Stream will be restarted and continue trying to send data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **String** | ID of a data stream. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_callback_secret

> delete_callback_secret(api_secret_id)
Delete a Callback Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_secret_id** | **i32** | A numeric ID of an API Secret | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_callback_url

> delete_callback_url(api_callback_id)
Delete a Callback URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_callback_id** | **i32** | A numeric ID of an API Callback URL | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_streamer_v2

> delete_data_streamer_v2(data_stream_id)
Delete Existing Data Stream

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **String** | ID of a data stream. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_callback_secret

> Vec<crate::models::GetApiCallbackSecret200ResponseInner> get_api_callback_secret()
List API Callback Secrets

Lists API callback secrets.  __Note:__ The `secret` property itself is not returned in this call. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GetApiCallbackSecret200ResponseInner>**](GetAPICallbackSecret_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_callback_urls

> Vec<serde_json::Value> get_api_callback_urls()
Retrieve list of API Callback URLs

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_callback_secretby_id

> crate::models::GetApiCallbackSecret200ResponseInner get_callback_secretby_id(api_secret_id)
Get a Callback Secret by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_secret_id** | **i32** | A numeric ID of an API secret | [required] |

### Return type

[**crate::models::GetApiCallbackSecret200ResponseInner**](GetAPICallbackSecret_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_callback_ur_lby_id

> serde_json::Value get_callback_ur_lby_id(api_callback_id)
Get a Callback URL by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_callback_id** | **i32** | A numeric ID of an API Callback URL | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_streamer_by_id_v2

> crate::models::GetDataStreamerByIdV2200Response get_data_streamer_by_id_v2(data_stream_id)
Get Details on Existing Data Stream

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **String** | ID of a data stream. | [required] |

### Return type

[**crate::models::GetDataStreamerByIdV2200Response**](GetDataStreamerByIdV2_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_streamer_data_stream_type

> Vec<crate::models::GetDataStreamerStatuses200ResponseInner> get_data_streamer_data_stream_type()
Get Data Stream Types

List available Data Stream Types.  (`1`) will only stream `Usage` Data.  (`2`) will only stream `Event` Data. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GetDataStreamerStatuses200ResponseInner>**](GetDataStreamerStatuses_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_streamer_enum_filter_field_type

> Vec<crate::models::GetDataStreamerStatuses200ResponseInner> get_data_streamer_enum_filter_field_type()
Get Possible Data Stream Filter Fields

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GetDataStreamerStatuses200ResponseInner>**](GetDataStreamerStatuses_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_streamer_statuses

> Vec<crate::models::GetDataStreamerStatuses200ResponseInner> get_data_streamer_statuses()
Get Possible Data Stream Statuses

List Possible Data Stream Statuses.  (`1`) is the normal `Running` state meaning your data stream is active and streams data.  (`2`) indicates a `Paused` data stream. It will not send messages to your destination. However, messages meant for this data stream are routed and will be stored up to 7 days. You will first receive historic data once you resume a paused streamer after longer pausing.  (`3`) indicates that an `Error` has occurred. This might be problems in the configuration (e.g. missing permissions) or your destination has been unavailable for a longer period of time and the data stream exceeded maximum retries. You may restart the streamer in order to fix a temporary problem, misconfigured streamers will continue to fail.  (`4`) is a `Pending` state. Your data stream has not been initialized, yet. This state may occur for a very short time. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GetDataStreamerStatuses200ResponseInner>**](GetDataStreamerStatuses_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_streamer_v2s

> crate::models::ListDataStreamerV2s200Response list_data_streamer_v2s()
List Data Stream configurations of your organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDataStreamerV2s200Response**](ListDataStreamerV2s_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v2_data_stream

> patch_v2_data_stream(data_stream_id, patch_v2_data_stream_request)
Modify Existing Data Stream

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **String** | ID of a data stream. | [required] |
**patch_v2_data_stream_request** | [**PatchV2DataStreamRequest**](PatchV2DataStreamRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

