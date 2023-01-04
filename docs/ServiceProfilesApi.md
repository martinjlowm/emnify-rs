# \ServiceProfilesApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_traffic_limit**](ServiceProfilesApi.md#add_traffic_limit) | **PUT** /api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id} | Add Traffic Limit to Service Profile
[**delete_quota_by_service_profile_id**](ServiceProfilesApi.md#delete_quota_by_service_profile_id) | **DELETE** /api/v1/service_profile/{profile_id}/quota/{quota_type} | Remove all quotas of assigned endpoints
[**remove_traffic_limit**](ServiceProfilesApi.md#remove_traffic_limit) | **DELETE** /api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id} | Remove Traffic Limit from a Service Profile
[**service_profile_by_profile_id_delete**](ServiceProfilesApi.md#service_profile_by_profile_id_delete) | **DELETE** /api/v1/service_profile/{profile_id} | Delete a Service Profile
[**service_profile_by_profile_id_get**](ServiceProfilesApi.md#service_profile_by_profile_id_get) | **GET** /api/v1/service_profile/{profile_id} | Retrieve a Service Profile
[**service_profile_by_profile_id_patch**](ServiceProfilesApi.md#service_profile_by_profile_id_patch) | **PATCH** /api/v1/service_profile/{profile_id} | Update Service Profile
[**service_profile_get**](ServiceProfilesApi.md#service_profile_get) | **GET** /api/v1/service_profile | List Service Profiles
[**service_profile_post**](ServiceProfilesApi.md#service_profile_post) | **POST** /api/v1/service_profile | Create Service Profile
[**service_profile_service_by_profile_and_service_delete**](ServiceProfilesApi.md#service_profile_service_by_profile_and_service_delete) | **DELETE** /api/v1/service_profile/{profile_id}/service/{service_id} | Remove a Service from a Service Profile
[**service_profile_service_by_profile_and_service_put**](ServiceProfilesApi.md#service_profile_service_by_profile_and_service_put) | **PUT** /api/v1/service_profile/{profile_id}/service/{service_id} | Add a Service to a Service Profile



## add_traffic_limit

> add_traffic_limit(profile_id, limit_id, service_id)
Add Traffic Limit to Service Profile

Add traffic limit to a collection of traffic limits associated with a service profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**limit_id** | **f32** | Numerical ID of a Traffic Limit | [required] |
**service_id** | **f32** | The Numeric ID of a Service, may be:  * `0`  - USSD  * `3`  - Voice, __warning:__ voice services are not available!  * `6`  - SMS MT  * `32` - SMS MO  * `38` - Data  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quota_by_service_profile_id

> delete_quota_by_service_profile_id(profile_id, quota_type)
Remove all quotas of assigned endpoints

Remove all quotas of endpoints which are assigned to this profile. Notice that `apply_data_quota` and/or `apply_sms_quota` have to be patched to false before, otherwise the endpoints will get blocked for having no quota left. This call will also return successfully if no endpoint is assigned or no quotas are set. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**quota_type** | **String** | The type of service of the quota, can be `data` or `sms` | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_traffic_limit

> remove_traffic_limit(profile_id, limit_id, service_id)
Remove Traffic Limit from a Service Profile

Removes the assignment of a Traffic Limit from a service associated with a service profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**limit_id** | **f32** | Numerical ID of a Traffic Limit | [required] |
**service_id** | **f32** | The Numeric ID of a Service, may be:  * `0`  - USSD  * `3`  - Voice, __warning:__ voice services are not available!  * `6`  - SMS MT  * `32` - SMS MO  * `38` - Data  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_by_profile_id_delete

> service_profile_by_profile_id_delete(profile_id)
Delete a Service Profile

Deletes a service profile and all its associations with services and traffic limits. A service profile can only be deleted if it is **not** selected on an endpoint. If it is not selected on an endpoint, the used_count is 0. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_by_profile_id_get

> crate::models::RetrieveaSingleServiceProfileresponse service_profile_by_profile_id_get(profile_id)
Retrieve a Service Profile

Retrieve a service profile with a given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |

### Return type

[**crate::models::RetrieveaSingleServiceProfileresponse**](RetrieveaSingleServiceProfileresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_by_profile_id_patch

> serde_json::Value service_profile_by_profile_id_patch(profile_id, update_service_profile)
Update Service Profile

Update a service profile with a given id.  Editable fields: * `name` (String optional) * `description` (String optional) * `allowed_3g` (boolean optional) * `allowed_4g` (boolean optional) * `allowed_nb_iot` (boolean optional) * __DEPRECATED__ `apply_quota` (boolean optional, defaults to false). Use `apply_data_quota` instead. Will be ignored if `apply_data_quota` is present. * `apply_data_quota` (boolean optional, defaults to false) * `apply_sms_quota` (boolean optional, defaults to false) * `retail` (boolean optional, defaults to false) * `sms_p2p_int` (boolean optional, defaults to true) * `sms_p2p_ext` (boolean optional, defaults to true) * `prepaid` (boolean optional, defaults to false) * `nipdp` (boolean optional, defaults to false) * `api_callback` (object optional) can be emptied by setting the id to null (\"api_callback\":{\"id\":null}) * `api_secret` (object optional) can be emptied by setting the id to null (\"api_secret\":{\"id\":null}) * `moc_callback` (object optional) can be emptied by setting the id to null (\"moc_callback\":{\"id\":null}) * `esme_interface_type` (object optional) * `breakout_region` (object optional) * `dns` (object optional) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**update_service_profile** | [**UpdateServiceProfile**](UpdateServiceProfile.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_get

> Vec<crate::models::RetrieveServiceProfileListresponse> service_profile_get()
List Service Profiles

Retrieves a collection of all Service Profiles for your organisation.  Returned service profiles contain just the total count of associated services, so this is the short view. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveServiceProfileListresponse>**](RetrieveServiceProfileListresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_post

> service_profile_post(createa_service_profilerequest)
Create Service Profile

Creates a new Service Profile. A `name` must be specified for the Service Profile and all other fields are optional.  Editable fields: * `name` (String required) * `description` (String optional) * `allowed_3g` (boolean optional, defaults to true) * `allowed_4g` (boolean optional, defaults to true) * `allowed_nb_iot` (boolean optional, defaults to true) * **DEPRECATED** `apply_quota` (boolean optional, defaults to false).  Use `apply_data_quota` instead. Will be ignored if `apply_data_quota` is present. * `apply_data_quota` (boolean optional, defaults to false) * `apply_sms_quota` (boolean optional, defaults to false) * `retail` (boolean optional, defaults to false) * `sms_p2p_int` (boolean optional, defaults to true) * `sms_p2p_ext` (boolean optional, defaults to true) * `prepaid` (boolean optional, defaults to false) * `nipdp` (boolean optional, defaults to false) * `api_callback` (object optional) * `api_secret` (object optional) * `moc_callback` (object optional) * `esme_interface_type` (object optional) * `breakout_region` (object optional) * `dns` (object optional)  __Note:__ enabling services (SMS, Data etc.) is done via `PUT` to `/api/v1/service_profile/{profile_id}/service/{service_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**createa_service_profilerequest** | [**CreateaServiceProfilerequest**](CreateaServiceProfilerequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_service_by_profile_and_service_delete

> service_profile_service_by_profile_and_service_delete(profile_id, service_id)
Remove a Service from a Service Profile

Remove service from the collection of services associated to a profile. A service can only be removed if it has no assigned traffic limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**service_id** | **f32** | The Numeric ID of a Service, may be:  * `0`  - USSD  * `3`  - Voice, __warning:__ voice services are not available!  * `6`  - SMS MT  * `32` - SMS MO  * `38` - Data  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_profile_service_by_profile_and_service_put

> service_profile_service_by_profile_and_service_put(profile_id, service_id)
Add a Service to a Service Profile

Add service to the collection of services associated to a profile. Multiple services can be assigned, but each only once.  __Warning:__ Adding the voice service with an id of `3` to a service profile will be successful, but this feature is __not enabled__ by the platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **f32** | The Numeric ID of a Service Profile | [required] |
**service_id** | **f32** | The Numeric ID of a Service, may be:  * `0`  - USSD  * `3`  - Voice, __warning:__ voice services are not available!  * `6`  - SMS MT  * `32` - SMS MO  * `38` - Data  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

