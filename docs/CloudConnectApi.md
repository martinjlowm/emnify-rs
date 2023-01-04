# \CloudConnectApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cloud_connect_attachment_tgw**](CloudConnectApi.md#create_cloud_connect_attachment_tgw) | **POST** /api/v1/cnc/breakout/tgw | Create a CloudConnect attachment via Transit Gateway
[**create_cloud_connect_attachment_vpn**](CloudConnectApi.md#create_cloud_connect_attachment_vpn) | **POST** /api/v1/cnc/breakout/vpn | Create a CloudConnect attachment via IPSec VPN
[**delete_cloud_connect_attachment**](CloudConnectApi.md#delete_cloud_connect_attachment) | **DELETE** /api/v1/cnc/breakout/{cloudconnect_attachment_id} | Delete a specific CloudConnect attachment
[**get_cloud_connect_attachment_by_id**](CloudConnectApi.md#get_cloud_connect_attachment_by_id) | **GET** /api/v1/cnc/breakout/{cloudconnect_attachment_id} | View details of a CloudConnect attachment
[**get_cloud_connect_attachments**](CloudConnectApi.md#get_cloud_connect_attachments) | **GET** /api/v1/cnc/breakout | List all CloudConnect attachments of an organisation
[**get_cloud_connect_available_regions_by_breakout_type**](CloudConnectApi.md#get_cloud_connect_available_regions_by_breakout_type) | **GET** /api/v1/cnc/breakout_type/{breakout_type_id}/available_region | Get list of available CloudConnect regions by breakout type
[**get_cloud_connect_breakout_types**](CloudConnectApi.md#get_cloud_connect_breakout_types) | **GET** /api/v1/cnc/breakout_type | List CloudConnect breakout types
[**get_cloud_connect_regions**](CloudConnectApi.md#get_cloud_connect_regions) | **GET** /api/v1/cnc/region | Get list of available CloudConnect regions
[**list_cloud_connect_custom_prices**](CloudConnectApi.md#list_cloud_connect_custom_prices) | **GET** /api/v1/cnc/pricing | List CloudConnect prices
[**retry_cloud_connect_attachment**](CloudConnectApi.md#retry_cloud_connect_attachment) | **PATCH** /api/v1/cnc/breakout/{cloudconnect_attachment_id} | Retry creation of an expired TGW breakout



## create_cloud_connect_attachment_tgw

> create_cloud_connect_attachment_tgw(create_cloud_connect_tgw_request)
Create a CloudConnect attachment via Transit Gateway

Creates a Transit Gateway breakout towards the customer AWS account.  After the creation the following steps have to be made from the customer's AWS account:  1. Accept the resource share about the 'CloudConnect' TransitGateway 2. Create a Transit Gateway Attachment to the own VPC with the services to connect to  More Information about this can be found on the CloudConnect KnowledgeBase article. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_cloud_connect_tgw_request** | Option<[**CreateCloudConnectTgwRequest**](CreateCloudConnectTgwRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cloud_connect_attachment_vpn

> create_cloud_connect_attachment_vpn(create_cloud_connect_vpn_request)
Create a CloudConnect attachment via IPSec VPN

Creates a CloudConnect attachment via a specified VPN server. Currently Static and Dynamic (BGP) VPN attachments are available. The created connection will be available within 3-5 minutes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_cloud_connect_vpn_request** | Option<[**CreateCloudConnectVpnRequest**](CreateCloudConnectVpnRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cloud_connect_attachment

> delete_cloud_connect_attachment(cloudconnect_attachment_id)
Delete a specific CloudConnect attachment

Deletes the specified CloudConnect attachment.  Attachments cannot be deleted if they are in the state `7 (Deactivated)` or `8 (Rolling Back)` or if they are Classic attachments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloudconnect_attachment_id** | **f32** | Numerical ID of a CloudConnect attachment  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_connect_attachment_by_id

> Vec<crate::models::GetCloudConnectAttachmentByIdResponseInner> get_cloud_connect_attachment_by_id(cloudconnect_attachment_id)
View details of a CloudConnect attachment

Displays the details including configured CIDR blocks of a CloudConnect Attachment.  For active IPSec VPN attachments, the tunnel information (public IP, outisde address, inside CIDR, PSK, ASN) including metrics over the last hour (tunnel state, bytes in/out) will be displayed as well. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloudconnect_attachment_id** | **f32** | Numerical ID of a CloudConnect attachment  | [required] |

### Return type

[**Vec<crate::models::GetCloudConnectAttachmentByIdResponseInner>**](GetCloudConnectAttachmentByIdResponse_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_connect_attachments

> Vec<crate::models::GetCloudConnectAttachmentsResponseInner> get_cloud_connect_attachments(status, r#type, organisation)
List all CloudConnect attachments of an organisation

Lists all CloudConnect attachments of the logged in organisation if it is an Enterprise. For Master organisations it lists all attachments of the direct child organisations. Deleted attachments are left out by default.  Additionally the attachments can be filtered by status and type. Master organisations can also filter by organisation ID of their direct child organisations. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**f32**> | Numerical ID of the CloudConnect attachment status <br> * `1` - Pending Customer Action * `2` - Pending AWS Activation * `3` - Pending EGN Activation * `4` - Pending CRG Activation * `5` - Active * `6` - Deactivation Pending * `7` - Deactivated * `8` - Rolling Back  |  |
**r#type** | Option<**f32**> | Numerical ID of the CloudConnect attachment type <br> * `1` - Transit Gateway * `2` - IPSec VPN * `3` - IPSec VPN BGP * `4` - IPSec VPN BGP High Availability * `5` - Direct Connect * `6` - Shared * `7` - Transit Gateway (Classic) * `8` - IPSec VPN (Classic)  |  |
**organisation** | Option<**f32**> | Numerical ID of an Organisation |  |

### Return type

[**Vec<crate::models::GetCloudConnectAttachmentsResponseInner>**](GetCloudConnectAttachmentsResponse_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_connect_available_regions_by_breakout_type

> Vec<crate::models::CloudConnectAvailableRegionInner> get_cloud_connect_available_regions_by_breakout_type(breakout_type_id)
Get list of available CloudConnect regions by breakout type

Returns a list of regions available for use when creating new CloudConnect breakouts of the selected breakout type.  If the attachment region is peered to another one, the device breakout region will also be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**breakout_type_id** | **i32** | ID of a CloudConnect breakout type. Possible values can be retrieved with `GET /api/v1/cnc/breakout_type`.  | [required] |

### Return type

[**Vec<crate::models::CloudConnectAvailableRegionInner>**](CloudConnect_available_region_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_connect_breakout_types

> Vec<crate::models::CloudConnectBreakoutType> get_cloud_connect_breakout_types()
List CloudConnect breakout types

Lists all CloudConnect breakout types. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CloudConnectBreakoutType>**](CloudConnectBreakoutType.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_connect_regions

> get_cloud_connect_regions(r#type)
Get list of available CloudConnect regions

This endpoint has been deprecated. Please check `GET /api/v1/cnc/breakout_type/{breakout_type_id}/available_region` for the current version.  Returns a list of regions available for use when creating new CloudConnect breakouts. The result can be filtered by breakout type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**i32**> | Breakout type ID to filter the results for. Possible values can be retrieved with `GET /api/v1/cnc/breakout_type`.  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cloud_connect_custom_prices

> Vec<crate::models::ListCloudConnectCustomResponseInner> list_cloud_connect_custom_prices()
List CloudConnect prices

Returns a list of the configured monthly prices for CloudConnect breakouts.  Master organisation, Mobile Network Operators and Resellers will get the list of prices they have configured for their child organisations.  Enterprise organisations will get the list of prices that apply for them. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ListCloudConnectCustomResponseInner>**](ListCloudConnectCustomResponse_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_cloud_connect_attachment

> retry_cloud_connect_attachment(cloudconnect_attachment_id)
Retry creation of an expired TGW breakout

Retries the creation of a Transit Gateway breakout where the accept attachment state has expired, meaning the attachment on the customer's side has not been created in time.  Retry is only allowed if: - The breakout is of type `Transit Gateway (type_id: 1)` - The breakout is in status `Pending AWS Activation (status_id: 2)` - The accept attachment state has expired (7 days after creation) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloudconnect_attachment_id** | **f32** | Numerical ID of a CloudConnect attachment  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

