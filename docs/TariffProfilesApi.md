# \TariffProfilesApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tariff_profile_by_id_delete**](TariffProfilesApi.md#tariff_profile_by_id_delete) | **DELETE** /api/v1/tariff_profile/{tariff_profile_id} | Delete Tariff Profile
[**tariff_profile_by_id_get**](TariffProfilesApi.md#tariff_profile_by_id_get) | **GET** /api/v1/tariff_profile/{tariff_profile_id} | Tariff Profile Details
[**tariff_profile_by_id_patch**](TariffProfilesApi.md#tariff_profile_by_id_patch) | **PATCH** /api/v1/tariff_profile/{tariff_profile_id} | Update Tariff Profile
[**tariff_profile_coverage_by_tp_id_get**](TariffProfilesApi.md#tariff_profile_coverage_by_tp_id_get) | **GET** /api/v1/tariff_profile/{tariff_profile_id}/coverage | List Tariff Profile Coverage
[**tariff_profile_get**](TariffProfilesApi.md#tariff_profile_get) | **GET** /api/v1/tariff_profile | List Tariff Profiles
[**tariff_profile_inclusive_volume_assignment**](TariffProfilesApi.md#tariff_profile_inclusive_volume_assignment) | **PUT** /api/v1/tariff_profile/{tariff_profile_id}/inclusive_volume/{inclusive_volume_id} | Assign Inclusive Volume to Tarriff Profile
[**tariff_profile_inclusive_volume_remove_assignment**](TariffProfilesApi.md#tariff_profile_inclusive_volume_remove_assignment) | **DELETE** /api/v1/tariff_profile/{tariff_profile_id}/inclusive_volume/{inclusive_volume_id} | Unassign Inclusive Volume from Tariff Profile
[**tariff_profile_post**](TariffProfilesApi.md#tariff_profile_post) | **POST** /api/v1/tariff_profile | Create Tariff Profile
[**tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete**](TariffProfilesApi.md#tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete) | **DELETE** /api/v1/tariff_profile/{tariff_profile_id}/ratezone_selection/{ratezone_id} | Delete Ratezone from Tariff Profile
[**tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put**](TariffProfilesApi.md#tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put) | **PUT** /api/v1/tariff_profile/{tariff_profile_id}/ratezone_selection/{ratezone_id} | Assign Ratezone to Tarriff Profile



## tariff_profile_by_id_delete

> tariff_profile_by_id_delete(tariff_profile_id)
Delete Tariff Profile

Deletes tariff profile. Tariff profiles used by an endpoint (`used_count` > 0) cannot be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | The numeric ID of a Tariff Profile | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_by_id_get

> crate::models::TariffProfile1 tariff_profile_by_id_get(tariff_profile_id)
Tariff Profile Details

Returns tariff profile specified by id. This tariff profile also contains information about the currently valid ratezones of the tariff in the tariff profile and if the ratezone is selected in the tariff profile. It also contains applied custom rates for the included tariffs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | The numeric ID of a Tariff Profile | [required] |

### Return type

[**crate::models::TariffProfile1**](TariffProfile_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_by_id_patch

> tariff_profile_by_id_patch(tariff_profile_id, patch_tariff_profilerequest)
Update Tariff Profile

Patch the specified tariff profile.  You can provide following fields with this request:  * `name` (String optional) * `description` (String optional) * `tariff` (Object optional) If the tariff is changed, all selections of ratezones are removed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | The numeric ID of a Tariff Profile | [required] |
**patch_tariff_profilerequest** | [**PatchTariffProfilerequest**](PatchTariffProfilerequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_coverage_by_tp_id_get

> Vec<crate::models::RetrieveCoverageresponse> tariff_profile_coverage_by_tp_id_get(tariff_profile_id)
List Tariff Profile Coverage

Provides the list of countries where that tariff profile can be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | tariff profile ID | [required] |

### Return type

[**Vec<crate::models::RetrieveCoverageresponse>**](RetrieveCoverageresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_get

> Vec<crate::models::TariffProfile1> tariff_profile_get()
List Tariff Profiles

Returns the list of tariff profiles of the user's own organisation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TariffProfile1>**](TariffProfile_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_inclusive_volume_assignment

> tariff_profile_inclusive_volume_assignment(tariff_profile_id, inclusive_volume_id)
Assign Inclusive Volume to Tarriff Profile

Assignes an inclusive volume to the selected tariff profile. If no inclusive volume is assigned and the customer has multiple active inclusive volumes, the traffic for this tariff profile will be rated as \"Pay As You Go\".  The selected inclusive volume must not be expired and the tariff profile should not have an inclusive volume assigned yet. The tariff of the tariff profile and the inclusive volume ratezone have to match. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | Tariff Profile ID | [required] |
**inclusive_volume_id** | **f32** | Inclusive Volume ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_inclusive_volume_remove_assignment

> tariff_profile_inclusive_volume_remove_assignment(tariff_profile_id, inclusive_volume_id)
Unassign Inclusive Volume from Tariff Profile

Unassign the inclusive volume from the selected tariff profile. If the customer has multiple active inclusive volume, the traffic within that tariff profile will be rated as \"Pay As You Go\" afterwards. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | Tariff Profile ID | [required] |
**inclusive_volume_id** | **f32** | Inclusive Volume ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_post

> tariff_profile_post(create_tariff_profilerequest)
Create Tariff Profile

Create the specified tariff profile ID must not be specified, neither in query String, nor in JSON payload. You can provide following fields with this request:  * `name` (String required) * `description` (String optional) * `tariff` (Object required) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tariff_profilerequest** | [**CreateTariffProfilerequest**](CreateTariffProfilerequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete

> tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete(tariff_profile_id, ratezone_id)
Delete Ratezone from Tariff Profile

Remove previously selected ratezones from a tariff profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | tariff profile ID | [required] |
**ratezone_id** | **f32** | ratezone ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put

> tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put(tariff_profile_id, ratezone_id)
Assign Ratezone to Tarriff Profile

Only currently valid and active ratezones can be selected for a tariff profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tariff_profile_id** | **f32** | tariff profile ID | [required] |
**ratezone_id** | **f32** | ratezone ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

