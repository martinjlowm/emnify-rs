# \OrganisationApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_ratezone_inclusive_volume**](OrganisationApi.md#assign_ratezone_inclusive_volume) | **put** /api/v1/organisation/{org_id_or_my}/inclusive_volume/{inclusive_volume_id} | Assign a ratezone inclusive volume to an organisation
[**get_active_organisation_inclusive_volume**](OrganisationApi.md#get_active_organisation_inclusive_volume) | **get** /api/v1/organisation/{org_id_or_my}/inclusive_volume/active | Get list of active organisation inclusive volumes
[**my_organisation_get**](OrganisationApi.md#my_organisation_get) | **get** /api/v1/organisation/my | My Organisation Details
[**organisation_status_get**](OrganisationApi.md#organisation_status_get) | **get** /api/v1/organisation/status | Organisation Status
[**statistics_daily_by_id_get**](OrganisationApi.md#statistics_daily_by_id_get) | **get** /api/v1/stats/daily | Organisation Usage and Costs Statistics per day for the current month
[**update_organisation_tariff**](OrganisationApi.md#update_organisation_tariff) | **patch** /api/v1/organisation/{org_id}/tariff | Update assigned tariff



## assign_ratezone_inclusive_volume

> crate::models::InlineResponse201 assign_ratezone_inclusive_volume(org_id_or_my, inclusive_volume_id, inline_object3)
Assign a ratezone inclusive volume to an organisation

Assign a pre-configured ratezone inclusive volume to the specified organisation.  This can be done in self-service as well as for direct child organisations. Only postpaid customers can assign inclusive volumes in self-service, while parent organisations can assign inclusive volumes for both postpaid and prepaid child organisations. The inclusive volume will be charged and calculated against traffic charges accordingly in every bill that is refreshed from that point on. Inclusive volume is counted per device on a monthly basis, the data can be pooled as well or be treated per individual device.  Only inclusive volumes of ratezones belonging to the tariffs assigned to the selected organisation can be chosen.  Enterprise organisations are only allowed to upgrade to a higher inclusive volume if there is already one active in the current month.  The `start_date` and `end_date` of the inclusive volume denote the billing period which this volume applies to, where the start must be always the first day of a month, and the end the last day. Times of the fields will be stripped. Inclusive volumes can only be assigned for the current or a future month.  The request body for this entrypoint is optional. Default values are: * `start_date`: First day of the current month * `end_date`: `null` (Inclusive volume will run indefinitely until the end date is set.) * `pooled`: `true` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` | [required] |[default to my]
**inclusive_volume_id** | **f32** | Numerical ID of a ratezone inclusive volume. Can be retrieved using `GET /api/v1/tariff/{tariff_id}/ratezone/{ratezone_id}/inclusive_volume`.  | [required] |
**inline_object3** | Option<[**InlineObject3**](InlineObject3.md)> |  |  |

### Return type

[**crate::models::InlineResponse201**](inline_response_201.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_organisation_inclusive_volume

> Vec<crate::models::InlineResponse2007> get_active_organisation_inclusive_volume(org_id_or_my)
Get list of active organisation inclusive volumes

Returns of a list of inclusive volumes that are currently active in the current billing period for the selected organisation. This will not inclusive volumes that are already expired or have the start date set for a future billing period.  Only inclusive volumes for the own organisation or direct child organisations are accessible. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` | [required] |[default to my]

### Return type

[**Vec<crate::models::InlineResponse2007>**](inline_response_200_7.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## my_organisation_get

> crate::models::InlineResponse200 my_organisation_get()
My Organisation Details

You can retrieve details about your own organisation

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisation_status_get

> Vec<crate::models::RetrieveOrganisationStatusesresponse> organisation_status_get()
Organisation Status

Provides the list of available organisation status (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveOrganisationStatusesresponse>**](RetrieveOrganisationStatusesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_daily_by_id_get

> Vec<serde_json::Value> statistics_daily_by_id_get()
Organisation Usage and Costs Statistics per day for the current month

Retrieve usage and costs statistics for the currently logged in organisation and all their child organisations accumulated per days from beginning of the month until today. Data traffic costs will not be returned for enterprise organisations with inclusive volume. 

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


## update_organisation_tariff

> update_organisation_tariff(org_id, update_tariff_request)
Update assigned tariff

Allows an organisation to change their assigned tariff.  The following restrictions apply: * The organisation must not have more than one tariff assigned. * The tariff can be changed only for the own organisation and direct child organisations. The organisation status must be \"ACTIVE\". * If performing the change in self-service, the organisation needs to be in evaluation. Parent organisations are allowed to change the tariffs of production organisations. * The selected tariff must be active, visible to the requested organisation and must have the same currency. Parent organisations are allowed to assign private tariffs to their child organisations.  In addition to updating the assigned tariff, the system will also: * Update all of the organisations tariff profiles to use the new tariff. * Activate all ratezones in the tariff in order to ensure that the endpoints do not lose connection. * Expire all currently active inclusive volumes and custom rates and delete future ones. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **f32** | Numerical ID of an Organisation | [required] |
**update_tariff_request** | [**UpdateTariffRequest**](UpdateTariffRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

