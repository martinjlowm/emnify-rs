# \OperatorApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**operator_get**](OperatorApi.md#operator_get) | **GET** /api/v1/operator | List Operators



## operator_get

> Vec<crate::models::OperatorGet200ResponseInner> operator_get()
List Operators

Returns a collection of existing operators.  It will return an array of items with following properties:  * `id`: identifier of this operator * `name`: the official name of the operator * `name_and_country`: Name / Country (for easy access in the UI) * `country`: country object with the nested country `id` * `mnc`: Array of Mobile Network Codes (MNCs) * `tapcode`: Array of TAP Codes 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::OperatorGet200ResponseInner>**](OperatorGet_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

