# \SimApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**register_sim_batch_by_bic**](SimApi.md#register_sim_batch_by_bic) | **PATCH** /api/v1/sim_batch/bic/{bic} | Register a given batch by BIC
[**s_im_statistics_daily_by_id_get**](SimApi.md#s_im_statistics_daily_by_id_get) | **GET** /api/v1/sim/{sim_id}/stats/daily | SIM Usage and Costs Statistics per day
[**sim_by_id_delete**](SimApi.md#sim_by_id_delete) | **DELETE** /api/v1/sim/{sim_id} | Delete a SIM
[**sim_by_id_get**](SimApi.md#sim_by_id_get) | **GET** /api/v1/sim/{sim_id} | SIM Details
[**sim_by_id_patch**](SimApi.md#sim_by_id_patch) | **PATCH** /api/v1/sim/{sim_id} | Update a SIM
[**sim_event_page_per_page_sort_by_sim_id_and_q_get**](SimApi.md#sim_event_page_per_page_sort_by_sim_id_and_q_get) | **GET** /api/v1/sim/{sim_id}/event | List SIM Events
[**sim_per_page_sort_by_q_and_page_get**](SimApi.md#sim_per_page_sort_by_q_and_page_get) | **GET** /api/v1/sim | List SIMs
[**sim_stats_by_id_get**](SimApi.md#sim_stats_by_id_get) | **GET** /api/v1/sim/{sim_id}/stats | SIM Usage and Costs Statistics
[**sim_status_get**](SimApi.md#sim_status_get) | **GET** /api/v1/sim/status | List SIM Statuses
[**validate_sim_batch_by_bic**](SimApi.md#validate_sim_batch_by_bic) | **GET** /api/v1/sim_batch/bic/{bic} | Validate if a given batch can be registered by BIC



## register_sim_batch_by_bic

> crate::models::SuccessfulBatchActivation register_sim_batch_by_bic(bic, activate_batch)
Register a given batch by BIC

Registers the given SIM batch, assigns all SIMs to the organisation, and sets the status of all contained SIMs to the given value. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bic** | **String** | The bic of the sim batch | [required] |
**activate_batch** | Option<[**ActivateBatch**](ActivateBatch.md)> |  |  |

### Return type

[**crate::models::SuccessfulBatchActivation**](Successful_batch_activation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## s_im_statistics_daily_by_id_get

> serde_json::Value s_im_statistics_daily_by_id_get(sim_id, start_date, end_date)
SIM Usage and Costs Statistics per day

Retrieve usage and costs statistics accumulated per days. Per default, the statistics for the current month will be returned. Data traffic costs can only be retrieved for organisations without inclusive volume.  To filter the statistics over a time range, a `start_date` and an `end_date` can be provided as query parameters. The filters have to be provided in `<property>=<value>` format.  When defining only the `start_date`, the statistics from that date until the end of the selected month will be returned.  Example request: `/api/v1/sim/123/stats/daily?end_date=2019-03-21&start_date=2019-03-01` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **f32** | SIM ID | [required] |
**start_date** | Option<**String**> | Filters the returned data by a start date.  |  |
**end_date** | Option<**String**> | Returned data will be filtered to only show results occuring before the end date  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_by_id_delete

> sim_by_id_delete(sim_id)
Delete a SIM

**Warning: Deleted SIMs cannot be recovered!**  Deletes a SIM. The following restrictions apply to deleting SIMs:  * SIMs with an endpoint assigned cannot be deleted. * A reseller may not delete SIMs they have sold. If the SIM `reseller_org_id` field is not empty and matches the organization ID making the request, a `403` error will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **i32** | SIM ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_by_id_get

> crate::models::SimEntry sim_by_id_get(sim_id)
SIM Details

Retrieve SIM details for a given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **i32** | SIM ID | [required] |

### Return type

[**crate::models::SimEntry**](Sim_Entry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_by_id_patch

> sim_by_id_patch(sim_id, update_sim)
Update a SIM

Update SIM resource.  You can provide the following fields with this request: * `issuer_organisation` (Object optional) - can be changed to a direct child organisation of appropriate type * `reseller_organisation` (Object optional) - can be changed to a direct child organisation of appropriate type or emptied (`\"reseller_org\":{\"id\": null}` or `\"reseller_org\":{}`) * `customer_organisation` (Object optional) - can be changed to own organisation or a direct child organisation of type \"Enterprise\" or emptied (`\"customer_org\":{\"id\": null}` or `\"customer_org\":{}`) * `status` (Object optional)  #### Notes on update restrictions:  * A user of the Issuer organisation can update any of the updateable fields * A user of the Reseller organisation can update the fields: reseller_org, customer_org * A user of the Customer organisation can only update the status field * The issuer_org can be updated to a child organisation of type \"Mobile Network Operator\" or \"Service Provider\" * The reseller_org can be updated to a child organisation of type \"Mobile Network Operator\" or \"Service Provider\" or \"Reseller\" * The customer_org can be updated to a child organisation of type \"Enterprise\" * The SIM status can be updated from id: 0 (\"Issued\") to either id: 1 (\"Activated\") or to id: 4 (\"Factory Test\") * The SIM status can be updated from id: 4 (\"Factory Test\") only to id: 1 (\"Activated\") * The SIM status can also be updated between id: 1 (\"Activated\") and id: 2 (\"Suspended\") back and forth 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **i32** | SIM ID | [required] |
**update_sim** | [**UpdateSim**](UpdateSim.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_event_page_per_page_sort_by_sim_id_and_q_get

> Vec<crate::models::RetrieveEventsresponse> sim_event_page_per_page_sort_by_sim_id_and_q_get(sim_id, page, per_page, sort, q)
List SIM Events

Returns the list of events, filtered, sorted and paged according to query parameters.  **CAUTION** This API endpoint deviates from the specified conventions and may not return the same HTTP Codes as the higher layer call (`/api/v1/sim/{sim_id}`). In case the requested `{sim_id}` does not exist or is not accessible for the user, **HTTP 200** will be returned with empty **[]** as long as the provided `{sim_id}` is a number and all parameters are valid. Please take that into consideration when building automation on top of the error behaviour of this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **i32** | SIM ID | [required] |
**page** | Option<**i32**> | Current page number |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |
**sort** | Option<**String**> | Sort properties according to a comma separated list of accepted fields. Valid fields are:  * `id` - (**event id**) * `timestamp` - (**event timestamp**) * `source` - (**event source**) * `severity` - (**event severity**) * `alert` - (**alert status**) * `organisation` - (**organisation name**) * `user` - (**user id**) * `endpoint` - (**endpoint name**) * `tags` - (**endpoint tags**) * `ip_address` - (**endpoint ip_address**) * `iccid` - (**sim iccid**) * `imsi` - (**sim imsi**) * `type` - (**event type**)  |  |
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Multiple filters must be in the format of a comma separated list of the following fields:  * `type` (**event_type**, numerical) * `source` (**event_type**, numerical, e.g. 0 = Network), 1 = Policy Control, 2 = API) * `severity` (**event_severity**, numerical, e.g. 0 = Info, 1 = Warn), 2 = Critical) * `alert` (boolean, e.g. true, false) * `description` (**event description**, string) * `organisation` (**organisation name**, string) * `user` (**user name**, string) * `endpoint` (**endpoint name**, string) * `tags` (**endpoint tags**, string) * `ip_address` (**endpoint IP address**, valid IPv4/IPv6 address) * `imei` (**endpoint imei**, numerical string) * `iccid` (**sim iccid**, numerical string) * `imsi` (**sim imsi**, numerical string) * `from` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with until!__) * `until` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with from!__) * `timestamp` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, for querying events of 1 day, deprecated in future)  |  |

### Return type

[**Vec<crate::models::RetrieveEventsresponse>**](RetrieveEventsresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_per_page_sort_by_q_and_page_get

> Vec<crate::models::SimEntry> sim_per_page_sort_by_q_and_page_get(page, per_page, q, sort)
List SIMs

Returns a list of SIMs, filtered, sorted and paged according to query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Current page number |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Expects comma separated list out of the allowed fields, e.g. * `id` * `issuer_org` * `reseller_org` * `customer_org` * `iccid` * `status` * `production_date` * `imsi` * `msisdn` * `endpoint` * `model`  |  |
**sort** | Option<**String**> | Sort properties according to comma separated list out of the allowed fields:  * `id` * `issuer_org` * `reseller_org` * `customer_org` * `iccid` * `status` * `production_date` * `endpoint` * `model`  Example to sort by status descending and id ascending: `sort=-status,id`  |  |

### Return type

[**Vec<crate::models::SimEntry>**](Sim_Entry.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_stats_by_id_get

> Vec<crate::models::ResponseSchemaForSimStatistics> sim_stats_by_id_get(sim_id)
SIM Usage and Costs Statistics

Retrieve usage and costs statistics for current/last month/hour. Data traffic costs can only be retrieved for organisations without inclusive volume. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sim_id** | **f32** | SIM ID | [required] |

### Return type

[**Vec<crate::models::ResponseSchemaForSimStatistics>**](Response_schema_for_SIM_Statistics.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sim_status_get

> Vec<crate::models::ListofAllAvailableSimStatusesresponse> sim_status_get()
List SIM Statuses

Retrieves a list of available statuses of SIMs.  * The initial state after the SIM has been registered to an account is 'Issued'. * SIM cards in 'Issued' status can be tested by patching them to 'Factory test' status. * Once the SIM card is activated, the SIM status will change to 'Activated'. * When you suspend the SIM its status is set to 'Suspended'. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ListofAllAvailableSimStatusesresponse>**](ListofAllAvailableSIMStatusesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_sim_batch_by_bic

> crate::models::ResponseOfActivatableSimBatch validate_sim_batch_by_bic(bic)
Validate if a given batch can be registered by BIC

Checks the given BIC code and the contained SIMs if they can be registered. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bic** | **String** | The bic of the sim batch | [required] |

### Return type

[**crate::models::ResponseOfActivatableSimBatch**](Response_of_activatable_SIM_batch.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

