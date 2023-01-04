# \TariffPlansApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organisation_active_tariff_plan**](TariffPlansApi.md#get_organisation_active_tariff_plan) | **GET** /api/v1/organisation/{org_id_or_my}/tariff_plan/active | Get the active tariff plan for the given organisation
[**organisation_tariff_plan_by_org_id_get**](TariffPlansApi.md#organisation_tariff_plan_by_org_id_get) | **GET** /api/v1/organisation/{org_id}/tariff_plan | List Organisation Tariff Plans



## get_organisation_active_tariff_plan

> crate::models::GetOrganisationActiveTariffPlanResponse get_organisation_active_tariff_plan(org_id_or_my)
Get the active tariff plan for the given organisation

Retrieve detailed information about the currently assigned `tariff_plan` of the given organisation. It can either be accessed for the own organisation or a direct child organisation. The `applied_price` field is calculated using the currently active SIM cards for the month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id_or_my** | **String** | Numerical ID of an Organisation or the string `my` to use the currently authorized organisation | [required] |[default to my]

### Return type

[**crate::models::GetOrganisationActiveTariffPlanResponse**](GetOrganisationActiveTariffPlanResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisation_tariff_plan_by_org_id_get

> Vec<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInner> organisation_tariff_plan_by_org_id_get(org_id)
List Organisation Tariff Plans

Retrieve a list of all Tariff Plans that are available for the given organisation. The available tariff plans for one's own organisation can also be retrieved with a call to `/api/v1/organisation/my/tariff_plan` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **f32** | Numerical ID of an Organisation | [required] |

### Return type

[**Vec<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInner>**](OrganisationTariffPlanByOrgIdGet_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

