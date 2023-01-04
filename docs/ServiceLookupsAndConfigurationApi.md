# \ServiceLookupsAndConfigurationApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dns_by_id_delete**](ServiceLookupsAndConfigurationApi.md#dns_by_id_delete) | **DELETE** /api/v1/dns/{dns_id} | Delete DNS config
[**dns_get**](ServiceLookupsAndConfigurationApi.md#dns_get) | **GET** /api/v1/dns | List DNS Configs
[**dns_post**](ServiceLookupsAndConfigurationApi.md#dns_post) | **POST** /api/v1/dns | Create DNS config
[**service_get**](ServiceLookupsAndConfigurationApi.md#service_get) | **GET** /api/v1/service | List Services
[**service_traffic_limit_by_id_get**](ServiceLookupsAndConfigurationApi.md#service_traffic_limit_by_id_get) | **GET** /api/v1/service/{service_id}/traffic_limit | Get Service Traffic Limit
[**traffic_limit_get**](ServiceLookupsAndConfigurationApi.md#traffic_limit_get) | **GET** /api/v1/traffic_limit | List Traffic Limits



## dns_by_id_delete

> dns_by_id_delete(dns_id)
Delete DNS config

Delete a DNS configuration object by ID.  __NOTE:__ A DNS config object cannot be deleted if it is in use by at least one Service Profile. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_id** | **f32** | DNS configuration ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_get

> Vec<crate::models::RetrieveDnSlistresponseInner> dns_get()
List DNS Configs

Retrieves a list of DNS configurations. DNS settings can be applied to a service profile and endpoints which use this service profile will have the associated DNS settings applied. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveDnSlistresponseInner>**](RetrieveDNSlistresponse_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dns_post

> dns_post(createa_dn_sentryrequest)
Create DNS config

Creates a DNS configuration object. The DNS config can be applied to a service profile and all endpoints which use that service profile will have the associated DNS settings applied. DNS changes are instantly applied to any __new PDP context__; already connected devices with established PDPs will continue to use the previous nameserver config until the next time they reconnect.  Primary and secondary nameservers and IP version (`4` for IPV4 or `6` for IPV6) must be specified with this request.  __NOTE:__ The system currently falls back to IPV4 for the actually-enforced networking settings of endpoints, therefore __the IPV6 parameter will be ignored__ when provided. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**createa_dn_sentryrequest** | [**CreateaDnSentryrequest**](CreateaDnSentryrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_get

> Vec<crate::models::RetrieveAvailableServicesresponse> service_get()
List Services

Retrieves a collection of available services. Services are read only objects.  Service objects contain expanded traffic limit nested objects. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableServicesresponse>**](RetrieveAvailableServicesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_traffic_limit_by_id_get

> Vec<crate::models::ServiceTrafficLimitInner> service_traffic_limit_by_id_get(service_id)
Get Service Traffic Limit

Traffic limits are system configuration parameters defined for a single service. Traffic limits do not have direct effect, but have to be explicitly assigned to an endpoint or a service profile. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **f32** | The Numeric ID of a Service, may be:  * `0`  - USSD  * `3`  - Voice, __warning:__ voice services are not available!  * `6`  - SMS MT  * `32` - SMS MO  * `38` - Data  | [required] |

### Return type

[**Vec<crate::models::ServiceTrafficLimitInner>**](Service_Traffic_Limit_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## traffic_limit_get

> Vec<crate::models::RetrieveavailableTrafficLimitsresponse> traffic_limit_get()
List Traffic Limits

Retrieves a list of available traffic limits.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveavailableTrafficLimitsresponse>**](RetrieveavailableTrafficLimitsresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

