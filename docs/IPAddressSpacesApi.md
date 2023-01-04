# \IpAddressSpacesApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ip_address_space_available_by_ip_address_version_get**](IpAddressSpacesApi.md#ip_address_space_available_by_ip_address_version_get) | **GET** /api/v1/ip_address_space/available | Get Random Address Spaces
[**ip_address_space_by_id_delete**](IpAddressSpacesApi.md#ip_address_space_by_id_delete) | **DELETE** /api/v1/ip_address_space/{address_space_id} | Release an IP Address Space from an Organisation
[**ip_address_space_by_id_put**](IpAddressSpacesApi.md#ip_address_space_by_id_put) | **PUT** /api/v1/ip_address_space/{address_space_id} | Assign an IP Address Space to an Organisation
[**ip_address_space_get**](IpAddressSpacesApi.md#ip_address_space_get) | **GET** /api/v1/ip_address_space | List IP Address Spaces



## ip_address_space_available_by_ip_address_version_get

> Vec<crate::models::RetrieveAvailableAddressSpacesresponse> ip_address_space_available_by_ip_address_version_get(ip_address_version)
Get Random Address Spaces

Provides a list of 10 random available address spaces (unassigned to any organisation). As the list is generated for each request, two successive requests will have different results. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_address_version** | Option<**f32**> | filter by IPv4 which is default or by IPv6. Example: ip_address_version=4 or ip_address_version=6 |  |

### Return type

[**Vec<crate::models::RetrieveAvailableAddressSpacesresponse>**](RetrieveAvailableAddressSpacesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_address_space_by_id_delete

> ip_address_space_by_id_delete(address_space_id)
Release an IP Address Space from an Organisation

Release the IP address space from association with the user's organisation.  Note that IP address spaces can only be removed, if the IP address space is not used on any of the organisations endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_space_id** | **f32** | ID of the IP address space to be assigned | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_address_space_by_id_put

> ip_address_space_by_id_put(address_space_id)
Assign an IP Address Space to an Organisation

The IP address space is assigned to the user's organisation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_space_id** | **f32** | ID of the IP address space to be assigned | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_address_space_get

> Vec<crate::models::RetrieveownIpAddressSpacesresponse> ip_address_space_get()
List IP Address Spaces

Returns the list of IP address space for the requesting user's organisation

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveownIpAddressSpacesresponse>**](RetrieveownIPAddressSpacesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

