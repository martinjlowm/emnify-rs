# \OrganisationApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_ratezone_inclusive_volume**](OrganisationApi.md#assign_ratezone_inclusive_volume) | **PUT** /api/v1/organisation/{org_id_or_my}/inclusive_volume/{inclusive_volume_id} | Assign a ratezone inclusive volume to an organisation
[**get_active_organisation_inclusive_volume**](OrganisationApi.md#get_active_organisation_inclusive_volume) | **GET** /api/v1/organisation/{org_id_or_my}/inclusive_volume/active | Get list of active organisation inclusive volumes
[**get_organisation_daily_stats**](OrganisationApi.md#get_organisation_daily_stats) | **GET** /api/v1/organisation/{org_id_or_my}/stats/daily | Daily organisation traffic and cost statistics
[**get_organisation_hourly_stats**](OrganisationApi.md#get_organisation_hourly_stats) | **GET** /api/v1/organisation/{org_id_or_my}/stats/hourly | Hourly organisation traffic statistics
[**get_organisation_monthly_stats**](OrganisationApi.md#get_organisation_monthly_stats) | **GET** /api/v1/organisation/{org_id_or_my}/stats | Monthly organisation traffic and cost statistics
[**my_organisation_get**](OrganisationApi.md#my_organisation_get) | **GET** /api/v1/organisation/my | My Organisation Details
[**organisation_status_get**](OrganisationApi.md#organisation_status_get) | **GET** /api/v1/organisation/status | List Organisation Status
[**statistics_daily_by_id_get**](OrganisationApi.md#statistics_daily_by_id_get) | **GET** /api/v1/stats/daily | Organisation Usage and Costs Statistics per day for the current month
[**update_organisation_tariff**](OrganisationApi.md#update_organisation_tariff) | **PATCH** /api/v1/organisation/{org_id}/tariff | Update assigned tariff



## assign_ratezone_inclusive_volume

> crate::models::AssignRatezoneInclusiveVolume201Response assign_ratezone_inclusive_volume(org_id_or_my, inclusive_volume_id, assign_ratezone_inclusive_volume_request)
Assign a ratezone inclusive volume to an organisation

Assign a pre-configured ratezone inclusive volume to the specified organisation.  This can be done in self-service as well as for direct child organisations. Only postpaid customers can assign inclusive volumes in self-service, while parent organisations can assign inclusive volumes for both postpaid and prepaid child organisations. The inclusive volume will be charged and calculated against traffic charges accordingly in every bill that is refreshed from that point on. Inclusive volume is counted per device on a monthly basis, the data can be pooled as well or be treated per individual device.  Only inclusive volumes of ratezones belonging to the tariffs assigned to the selected organisation can be chosen.  Enterprise organisations are only allowed to upgrade to a higher inclusive volume if there is already one active in the current month.  The `start_date` and `end_date` of the inclusive volume denote the billing period which this volume applies to, where the start must be always the first day of a month, and the end the last day. Times of the fields will be stripped. Inclusive volumes can only be assigned for the current or a future month.  The request body for this entrypoint is optional. Default values are: * `start_date`: First day of the current month * `end_date`: `null` (Inclusive volume will run indefinitely until the end date is set.) * `pooled`: `true` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]
**inclusive_volume_id** | **f32** | Numerical ID of a ratezone inclusive volume. Can be retrieved using `GET /api/v1/tariff/{tariff_id}/ratezone/{ratezone_id}/inclusive_volume`.  | [required] |
**assign_ratezone_inclusive_volume_request** | Option<[**AssignRatezoneInclusiveVolumeRequest**](AssignRatezoneInclusiveVolumeRequest.md)> |  |  |

### Return type

[**crate::models::AssignRatezoneInclusiveVolume201Response**](AssignRatezoneInclusiveVolume_201_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_organisation_inclusive_volume

> Vec<crate::models::GetActiveOrganisationInclusiveVolumeResponseInner> get_active_organisation_inclusive_volume(org_id_or_my)
Get list of active organisation inclusive volumes

Returns of a list of inclusive volumes that are currently active in the current billing period for the selected organisation. This will not inclusive volumes that are already expired or have the start date set for a future billing period.  Only inclusive volumes for the own organisation or direct child organisations are accessible. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]

### Return type

[**Vec<crate::models::GetActiveOrganisationInclusiveVolumeResponseInner>**](GetActiveOrganisationInclusiveVolumeResponse_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisation_daily_stats

> Vec<crate::models::GetDailyOrganisationStatsResponseInner> get_organisation_daily_stats(org_id_or_my, start_date, end_date)
Daily organisation traffic and cost statistics

Retrieves traffic and cost statistics for the selected organisation and all their child organisations, accumulated per day. By default, statistics for the current month will be returned.  To filter the statistics over a time range, a `start_date` and an `end_date` can be provided as query parameters. The filters have to be provided in `<property>=<value>` format.  When defining only the `start_date`, the statistics from that date until the end of the selected month will be returned.  Example request: `/api/v1/organisation/123/stats/daily?end_date=2019-03-03&start_date=2019-03-01` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]
**start_date** | Option<**String**> | Filters the returned data by a start date. If no 'end date' is provided, data will be returned until the end of the selected month.  |  |
**end_date** | Option<**String**> | Returned data will be filtered to only show results occurring before the end date.  |  |

### Return type

[**Vec<crate::models::GetDailyOrganisationStatsResponseInner>**](Get_daily_organisation_stats_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisation_hourly_stats

> crate::models::GetHourlyOrganisationStatsResponse get_organisation_hourly_stats(org_id_or_my, start_date, end_date)
Hourly organisation traffic statistics

Retrieve traffic statistics for the selected organisation and all their child organisations accumulated per hour. Per default, the statistics of the last three hours will be returned.  To filter the statistics over a time range, a `start_date` and an `end_date` can be provided as query parameters. The filters have to be provided in `<property>=<value>` format.  Example request: `/api/v1/organisation/123/stats/hourly?end_date=2019-03-03&start_date=2019-03-01` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]
**start_date** | Option<**String**> | Filters the returned data by a start date. Will be set to current time - 3 hours if left empty.  |  |
**end_date** | Option<**String**> | Returned data will be filtered to only show results occurring before the end date. Will be set to current time + 1 hour if left empty.  |  |

### Return type

[**crate::models::GetHourlyOrganisationStatsResponse**](Get_hourly_organisation_stats_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisation_monthly_stats

> crate::models::GetMonthlyOrganisationStatsResponse get_organisation_monthly_stats(org_id_or_my)
Monthly organisation traffic and cost statistics

Retrieves traffic and cost statistics for the selected organisation.  Traffic statistics accumulate for the `current_month` and `last_month` of the selected organisation and all its child organisations. All other returned values (i.e., SIM statistics, user counts, etc.) are only for the selected organisation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]

### Return type

[**crate::models::GetMonthlyOrganisationStatsResponse**](Get_monthly_organisation_stats_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## my_organisation_get

> crate::models::MyOrganisationGet200Response my_organisation_get()
My Organisation Details

You can retrieve details about your own organisation

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MyOrganisationGet200Response**](MyOrganisationGet_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisation_status_get

> Vec<crate::models::RetrieveOrganisationStatusesresponse> organisation_status_get()
List Organisation Status

Provides a list of available organisation status (lookup).

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

