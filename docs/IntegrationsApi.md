# \IntegrationsApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_data_stream_filter**](IntegrationsApi.md#add_data_stream_filter) | **put** /api/v1/data_stream/{data_stream_id}/filter/event_type/{event_type_id} | Add event filter to a data stream
[**create_callback_secret**](IntegrationsApi.md#create_callback_secret) | **post** /api/v1/api_secret | Create a Callback Secret
[**create_callback_url**](IntegrationsApi.md#create_callback_url) | **post** /api/v1/api_callback | Create a Callback URL
[**create_data_stream**](IntegrationsApi.md#create_data_stream) | **post** /api/v1/data_stream | Create a Data Stream
[**delete_callback_secret**](IntegrationsApi.md#delete_callback_secret) | **delete** /api/v1/api_secret/{api_secret_id} | Delete a Callback Secret
[**delete_callback_url**](IntegrationsApi.md#delete_callback_url) | **delete** /api/v1/api_callback/{api_callback_id} | Delete a Callback URL
[**delete_data_stream_filter**](IntegrationsApi.md#delete_data_stream_filter) | **delete** /api/v1/data_stream/{data_stream_id}/filter/event_type/{event_type_id} | Delete a Data Stream filter
[**delete_data_streams**](IntegrationsApi.md#delete_data_streams) | **delete** /api/v1/data_stream/{data_stream_id} | Delete a Data Stream by ID
[**get_api_callback_secret**](IntegrationsApi.md#get_api_callback_secret) | **get** /api/v1/api_secret | List API Callback Secrets
[**get_api_callback_ur_ls**](IntegrationsApi.md#get_api_callback_ur_ls) | **get** /api/v1/api_callback | Retrieve list of API Callback URLs
[**get_callback_secretby_id**](IntegrationsApi.md#get_callback_secretby_id) | **get** /api/v1/api_secret/{api_secret_id} | Get a Callback Secret by ID
[**get_callback_ur_lby_id**](IntegrationsApi.md#get_callback_ur_lby_id) | **get** /api/v1/api_callback/{api_callback_id} | Get a Callback URL by ID
[**get_data_stream_filters**](IntegrationsApi.md#get_data_stream_filters) | **get** /api/v1/data_stream/{data_stream_id}/filter/event_type | Retrieve event filters of a datastream
[**get_data_streams**](IntegrationsApi.md#get_data_streams) | **get** /api/v1/data_stream | Retrieve List of Data Streams



## add_data_stream_filter

> add_data_stream_filter(data_stream_id, event_type_id)
Add event filter to a data stream

One or more filters by `event_type` can be added to a data stream. When such filters are applied, only events of those type are included in the data stream. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **i32** | Numerical ID of a Data Stream | [required] |
**event_type_id** | **f32** | Numerical ID of an event type. Event types and their description can be found in the `/api/v1/event/type` lookup call.  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_callback_secret

> create_callback_secret(inline_object2)
Create a Callback Secret

Creates a new secret that may be used by API callbacks.  When an `api_secret` is assigned to a Service Profile with an API callback, API requests towards this URL will contain an `Authorization` header with a JSON Web Token. The `api_secret` is used as the _Signing Key_ of the JWT. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | [required] |

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


## create_data_stream

> create_data_stream(inline_object1)
Create a Data Stream

 Data Streams may be created for real-time streaming of event and usage data in either `JSON` or `CSV` formats. The request body must contain the following properties:  * `stream_historic_data`: `0` - disabled, `1` - enabled, data up to 20 days old will be included in the stream * `data_stream_type`: an object that determines the type of data to be sent. The `id` property must be one of the following:     * `id: 1` - Usage Data     * `id: 2` - Event Data     * `id: 3` - Usage Data & Events * `api_type`: an object that indicates the integration type. Must be one of the following:     * `id: 1` - REST API     * `id: 2` - keen.io     * `id: 3` - DataDog     * `id: 4` - AWS Kinesis     * `id: 5` - _Deprecated_     * `id: 6` - REST API in Bulk Mode     * `id: 7` - Salesforce     * `id: 8` - AWS S3  ##### Additional Properties  The following additional parameters should be added depending on the data stream type:  * `api_parameter`: required for _AWS S3_ and for _AWS Kinesis_ in the format `<region>/<stream_or_bucket_name>` * `event_stream`: required for _Salesforce only_, the ID of the target event stream should be passed in here. * `api_username`: required for _Salesforce, keen.io, Datadog_. For _AWS_ integrations, this must be the ARN of the role with write permissions to the destination resource with a Trust Relationship applied. * `api_password`: required for _Salesforce, keen.io, Datadog_.  ##### API Callback URL  Data streams configured to use _Rest API_, _Rest API in Bulk Mode_ and _Salesforce_ may include an `api_callback` object (see  _/api/v1/api_callback_). The following properties may be included:  * `id` Integer (required): A numerical ID of an _existing API callback_ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object1** | Option<[**InlineObject1**](InlineObject1.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
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


## delete_data_stream_filter

> delete_data_stream_filter(data_stream_id, event_type_id)
Delete a Data Stream filter

Removes event filters applied to a data stream by `event_type` id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **i32** | Numerical ID of a Data Stream | [required] |
**event_type_id** | **f32** | Numerical ID of an event type. Event types and their description can be found in the `/api/v1/event/type` lookup call.  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_streams

> delete_data_streams(data_stream_id)
Delete a Data Stream by ID

A data stream may be deleted by ID. The ID is the top-level `id` property returned in each object listed in `GET /data_stream`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **i32** | Numerical ID of a Data Stream | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_callback_secret

> Vec<crate::models::InlineResponse2005> get_api_callback_secret()
List API Callback Secrets

Lists API callback secrets.  _Note:_ The `secret` property itself is not returned in this call. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2005>**](inline_response_200_5.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_callback_ur_ls

> Vec<serde_json::Value> get_api_callback_ur_ls()
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

> crate::models::InlineObject2 get_callback_secretby_id(api_secret_id)
Get a Callback Secret by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_secret_id** | **i32** | A numeric ID of an API secret | [required] |

### Return type

[**crate::models::InlineObject2**](inline_object_2.md)

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


## get_data_stream_filters

> Vec<crate::models::CreateMfaKeyResponseStatus> get_data_stream_filters(data_stream_id)
Retrieve event filters of a datastream

Returns a list of event filters applied to a data stream. When event filters are applied to data streams, only events of that type are included in a stream. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_stream_id** | **i32** | Numerical ID of a Data Stream | [required] |

### Return type

[**Vec<crate::models::CreateMfaKeyResponseStatus>**](Create_MFA_Key_Response_status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_streams

> Vec<crate::models::InlineResponse2004> get_data_streams()
Retrieve List of Data Streams

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2004>**](inline_response_200_4.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

