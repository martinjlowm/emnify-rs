# \UssdApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**endpoint_ussd_by_id_post**](UssdApi.md#endpoint_ussd_by_id_post) | **POST** /api/v1/endpoint/{endpoint_id}/ussd | Create USSD dialog



## endpoint_ussd_by_id_post

> crate::models::StartingaUssdDialogresponse endpoint_ussd_by_id_post(endpoint_id, startinga_ussd_dialogrequest)
Create USSD dialog

Your application may start a Network-Initiated (NI) USSD Dialog for an endpoint. If the endpoint has an IMSI online (location data is available and valid for one of its IMSIs), then a session-id is generated and returned. This session-id will be used in all subsequent USSD communication between your application and the endpoint. You must provide following fields with this request:  * `ussd-begin` (Object required) containing  \"type\" and \"message\" * `type` (String required) can be \"request\" or \"notification\" * `message` (Object required) including encoding (String optional) e.g. \"default\" (where default is gsm7) or \"ucs2\", body (String required).  Depending on the encoding, the maximum length of the body is 164 (default) or 72 (ucs2).  If the USSD Dialog cannot be initiated, possible errors include: * \"Unknown Subscriber Error\" * \"Endpoint Not Found\" 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**startinga_ussd_dialogrequest** | [**StartingaUssdDialogrequest**](StartingaUssdDialogrequest.md) |  | [required] |

### Return type

[**crate::models::StartingaUssdDialogresponse**](StartingaUSSDDialogresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

