# \EndpointApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_endpoint**](EndpointApi.md#create_endpoint) | **POST** /api/v1/endpoint | Create Endpoint
[**delete_endpoint_data_quota_by_id**](EndpointApi.md#delete_endpoint_data_quota_by_id) | **DELETE** /api/v1/endpoint/{endpoint_id}/quota/data | Remove Data Quota
[**delete_endpoint_sms_quota_by_id**](EndpointApi.md#delete_endpoint_sms_quota_by_id) | **DELETE** /api/v1/endpoint/{endpoint_id}/quota/sms | Remove SMS Quota
[**endpoint_balance_by_endpoint_id_delete**](EndpointApi.md#endpoint_balance_by_endpoint_id_delete) | **DELETE** /api/v1/endpoint/{endpoint_id}/balance | Reset Prepaid Balance
[**endpoint_balance_by_endpoint_id_get**](EndpointApi.md#endpoint_balance_by_endpoint_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/balance | Endpoint Prepaid Balance
[**endpoint_balance_by_endpoint_id_post**](EndpointApi.md#endpoint_balance_by_endpoint_id_post) | **POST** /api/v1/endpoint/{endpoint_id}/balance | Update Prepaid Balance
[**endpoint_by_id_delete**](EndpointApi.md#endpoint_by_id_delete) | **DELETE** /api/v1/endpoint/{endpoint_id} | Delete Endpoint
[**endpoint_by_id_get**](EndpointApi.md#endpoint_by_id_get) | **GET** /api/v1/endpoint/{endpoint_id} | Get Endpoint
[**endpoint_by_id_patch**](EndpointApi.md#endpoint_by_id_patch) | **PATCH** /api/v1/endpoint/{endpoint_id} | Update Endpoint
[**endpoint_connectivity_by_id_get**](EndpointApi.md#endpoint_connectivity_by_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/connectivity | Endpoint Connectivity Status
[**endpoint_events_by_id**](EndpointApi.md#endpoint_events_by_id) | **GET** /api/v1/endpoint/{endpoint_id}/event | List Endpoint events
[**endpoint_operator_blacklist_by_endpoint_id_get**](EndpointApi.md#endpoint_operator_blacklist_by_endpoint_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/operator_blacklist | List Operator Blacklist for Endpoint
[**endpoint_operator_blacklist_by_ep_id_and_operator_id_delete**](EndpointApi.md#endpoint_operator_blacklist_by_ep_id_and_operator_id_delete) | **DELETE** /api/v1/endpoint/{endpoint_id}/operator_blacklist/{operator_id} | Remove an Operator from the Blacklist
[**endpoint_operator_blacklist_by_ep_id_and_operator_id_put**](EndpointApi.md#endpoint_operator_blacklist_by_ep_id_and_operator_id_put) | **PUT** /api/v1/endpoint/{endpoint_id}/operator_blacklist/{operator_id} | Add an Operator to the Blacklist
[**endpoint_quota_data_by_endpoint_id_get**](EndpointApi.md#endpoint_quota_data_by_endpoint_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/quota/data | Retrieve Data Quota details
[**endpoint_quota_data_by_endpoint_id_post**](EndpointApi.md#endpoint_quota_data_by_endpoint_id_post) | **POST** /api/v1/endpoint/{endpoint_id}/quota/data | Set Data Quota
[**endpoint_quota_sms_by_endpoint_id_get**](EndpointApi.md#endpoint_quota_sms_by_endpoint_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/quota/sms | Show SMS Quota details
[**endpoint_quota_sms_by_endpoint_id_post**](EndpointApi.md#endpoint_quota_sms_by_endpoint_id_post) | **POST** /api/v1/endpoint/{endpoint_id}/quota/sms | Set SMS Quota
[**endpoint_sms_by_endpoint_id_and_sms_id_delete**](EndpointApi.md#endpoint_sms_by_endpoint_id_and_sms_id_delete) | **DELETE** /api/v1/endpoint/{endpoint_id}/sms/{sms_id} | Cancel SMS
[**endpoint_sms_by_endpoint_id_and_sms_id_get**](EndpointApi.md#endpoint_sms_by_endpoint_id_and_sms_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/sms/{sms_id} | SMS details
[**endpoint_sms_by_id_get**](EndpointApi.md#endpoint_sms_by_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/sms | List sent and received SMS
[**endpoint_sms_by_id_post**](EndpointApi.md#endpoint_sms_by_id_post) | **POST** /api/v1/endpoint/{endpoint_id}/sms | Send SMS to an Endpoint
[**endpoint_stats_by_id_get**](EndpointApi.md#endpoint_stats_by_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/stats | Endpoint Usage and Costs Statistics
[**endpoint_stats_daily_by_id_get**](EndpointApi.md#endpoint_stats_daily_by_id_get) | **GET** /api/v1/endpoint/{endpoint_id}/stats/daily | Endpoint Usage and Costs Statistics per day
[**endpoint_status_get**](EndpointApi.md#endpoint_status_get) | **GET** /api/v1/endpoint/status | List Endpoint Statuses
[**get_connectivity_info_by_endpoint_id**](EndpointApi.md#get_connectivity_info_by_endpoint_id) | **GET** /api/v1/endpoint/{endpoint_id}/connectivity_info | Connectivity Info of an Endpoint
[**get_endpoints**](EndpointApi.md#get_endpoints) | **GET** /api/v1/endpoint | List Endpoints
[**update_endpoint_connectivity_by_id**](EndpointApi.md#update_endpoint_connectivity_by_id) | **PATCH** /api/v1/endpoint/{endpoint_id}/connectivity | Reset Endpoint Connectivity



## create_endpoint

> create_endpoint(endpoint)
Create Endpoint

If a `sim` object is provided, the SIM with the contained ID will be assigned to the endpoint. The `activate` property defaults to `true` and can be omitted unless the SIM should not be activated with this API call.  The following fields may be provided: * `name` (String required) * `service_profile` (Object required) * `tariff_profile` (Object required) * `status` (Object required) - `0` = __Enabled__, `1` = __Disabled__! * `tags` (String optional) * `imei` (String optional) * `imei_lock` (Boolean optional) * `sim` (Object optional)   - `id` (number required) SIM ID to be assigned to this endpoint   - `activate` (Boolean, optional, default:true) * `ip_address` (String optional) * `ip_address_space` (Object, optional if IP address is omitted, mandatory when IP address is set) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | [**Endpoint**](Endpoint.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_endpoint_data_quota_by_id

> delete_endpoint_data_quota_by_id(endpoint_id)
Remove Data Quota

Will delete the data quota for the endpoint, if any is set. Note that if `apply_data_quota` is still set in the service profile, the endpoint will get blocked from data service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_endpoint_sms_quota_by_id

> delete_endpoint_sms_quota_by_id(endpoint_id)
Remove SMS Quota

Will delete the SMS quota for the endpoint, if any is set. Note that if `apply_sms_quota` is still set in the service profile, the endpoint will get blocked from sending sms. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_balance_by_endpoint_id_delete

> endpoint_balance_by_endpoint_id_delete(endpoint_id)
Reset Prepaid Balance

A `DELETE` request will reset the prepaid balance to a value of zero, in the current organisation currency. * Location updates will be rejected * PDP context requests will be rejected * MO-SMS will not be acknowledged and will time out 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_balance_by_endpoint_id_get

> crate::models::RetrievePrepaidBalanceresponse endpoint_balance_by_endpoint_id_get(endpoint_id)
Endpoint Prepaid Balance

Retrieves the balance of an Endpoint. The following properties are returned:  * `amount`: the current balance in currency with up to 6 decimal places of precision   * the amount may reach a negative value by deduction through the API or by deferred processing of usage charges   * If the amount is 0 or negative, the endpoint will be blocked from using teleservices, specifically     * Location updates will be rejected     * PDP context requests will be rejected     * MO-SMS will not be acknowledged and will time out * `currency`: a nested object with info on the currency represented by the `amount` * `auto_reset`: if enabled, the balance will be set automatically to the last loaded amount at the beginning of the month * `last_loaded_amount`: the amount of the last top-up/patch transaction 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**crate::models::RetrievePrepaidBalanceresponse**](RetrievePrepaidBalanceresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_balance_by_endpoint_id_post

> endpoint_balance_by_endpoint_id_post(endpoint_id, update_prepaid_balanceresponse)
Update Prepaid Balance

At any time, the prepaid balance of an endpoint can be updated either by adding or subtracting a certain amount. If the currency of the balance does not match the currency of the organisation, first a balance reset has to be issued. the currency doesn't have to be specified explicitly anymore.  The API expects the following parameters in the JSON body  * `amount` (number, required) - A positive or negative value with up to 6 decimal places of precision.  The new balance will be the sum of the old balance plus this amount. As given amounts may be negative, the result will be a deduction from the balance  * `expiry_date` (string, required) - A timestamp specifying an expiry date in the future 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**update_prepaid_balanceresponse** | Option<[**UpdatePrepaidBalanceresponse**](UpdatePrepaidBalanceresponse.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_by_id_delete

> endpoint_by_id_delete(endpoint_id)
Delete Endpoint

Deletes an endpoint and all its child entities. Please ensure the endpoint does not have a SIM assigned, otherwise the deletion will fail. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_by_id_get

> crate::models::Endpoint endpoint_by_id_get(endpoint_id)
Get Endpoint

Retrieves information on an endpoint with a given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**crate::models::Endpoint**](Endpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_by_id_patch

> endpoint_by_id_patch(endpoint_id, update_endpoint)
Update Endpoint

Updates the details of an endpoint.  You can provide following fields with this request: * `name` (String, optional) * `tags` (String, optional) * `status` (Object, optional) * `service profile` (Object, optional) * `tariff profile` (Object, optional) * `ip_address` (String, optional) * `ip_address_space` (Object optional if IP address is missing, mandatory when IP address is set) * `sim` (Object, optional) * `imei` (String, optional) * `imei_lock` (Boolean, optional) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**update_endpoint** | [**UpdateEndpoint**](UpdateEndpoint.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_connectivity_by_id_get

> crate::models::RetrieveEndpointConnectivityStatusresponse endpoint_connectivity_by_id_get(endpoint_id)
Endpoint Connectivity Status

Retrieve details about current connectivity status of endpoint. The following is a list of possible statuses:  * `ATTACHED`: The Endpoint has succesfully attached to the Home Core network in the past. The device will be shown as `ATTACHED` until the visited network has signaled that the device is inactive/offline. Usually the visited network informs the Core Network within 1-2 days after a device went offline.  * `ONLINE`: The Endpoint has an active data connection  * `OFFLINE`: The Endpoint has not attached to the core network yet or the device was previously attached but the visited network signaled that the device had no activity for the last 1-2 days. Note: The device is not reachable for external services (e.g. SMS, MSRN lookup).  * `BLOCKED`: The device is not granted service. Endpoints are assigned this status when they have exceeded traffic limits. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**crate::models::RetrieveEndpointConnectivityStatusresponse**](RetrieveEndpointConnectivityStatusresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_events_by_id

> crate::models::RetrieveEventsresponse endpoint_events_by_id(endpoint_id, page, per_page, sort, q)
List Endpoint events

Returns the list of events, filtered, sorted and paged according to query parameters.  **Note:** A full list of events is found in section \"Retrieve Event Types\" (`/api/v1/event/type`).  **CAUTION** This API endpoint deviates from the specified conventions and may not return the same HTTP Codes as the higher layer call (`/api/v1/endpoint/{endpoint_id}`). In case the requested `{endpoint_id}` does not exist or is not accessible for the user, **HTTP 200** will be returned with empty **[]** as long as the provided `{endpoint_id}` is a number and all parameters are valid. Please take that into consideration when building automation on top of the error behaviour of this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**page** | Option<**i32**> | Current page number |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |
**sort** | Option<**String**> | Sort properties according to a comma separated list of accepted fields. Valid fields are:  * `id` - (**event id**) * `timestamp` - (**event timestamp**) * `source` - (**event source**) * `severity` - (**event severity**) * `alert` - (**alert status**) * `organisation` - (**organisation name**) * `user` - (**user id**) * `endpoint` - (**endpoint name**) * `tags` - (**endpoint tags**) * `ip_address` - (**endpoint ip_address**) * `iccid` - (**sim iccid**) * `imsi` - (**sim imsi**) * `type` - (**event type**)  |  |
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Multiple filters must be in the format of a comma separated list of the following fields:  * `type` (**event_type**, numerical) * `source` (**event_type**, numerical, e.g. 0 = Network), 1 = Policy Control, 2 = API) * `severity` (**event_severity**, numerical, e.g. 0 = Info, 1 = Warn), 2 = Critical) * `alert` (boolean, e.g. true, false) * `description` (**event description**, string) * `organisation` (**organisation name**, string) * `user` (**user name**, string) * `endpoint` (**endpoint name**, string) * `tags` (**endpoint tags**, string) * `ip_address` (**endpoint IP address**, valid IPv4/IPv6 address) * `imei` (**endpoint imei**, numerical string) * `iccid` (**sim iccid**, numerical string) * `imsi` (**sim imsi**, numerical string) * `from` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with until!__) * `until` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with from!__) * `timestamp` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, for querying events of 1 day, deprecated in future)  |  |

### Return type

[**crate::models::RetrieveEventsresponse**](RetrieveEventsresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_operator_blacklist_by_endpoint_id_get

> Vec<crate::models::RetrieveOperatorBlacklistresponse> endpoint_operator_blacklist_by_endpoint_id_get(endpoint_id)
List Operator Blacklist for Endpoint

Returns a list of blacklisted Operators for the requested Endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**Vec<crate::models::RetrieveOperatorBlacklistresponse>**](RetrieveOperatorBlacklistresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_operator_blacklist_by_ep_id_and_operator_id_delete

> endpoint_operator_blacklist_by_ep_id_and_operator_id_delete(endpoint_id, operator_id)
Remove an Operator from the Blacklist

Remove Operator from the Blacklist of an Endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**operator_id** | **f32** | Numerical ID of an Operator | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_operator_blacklist_by_ep_id_and_operator_id_put

> endpoint_operator_blacklist_by_ep_id_and_operator_id_put(endpoint_id, operator_id)
Add an Operator to the Blacklist

Adds an Operator to the Blacklist of an Endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**operator_id** | **f32** | Numerical ID of an Operator | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_quota_data_by_endpoint_id_get

> crate::models::EndpointQuota endpoint_quota_data_by_endpoint_id_get(endpoint_id)
Retrieve Data Quota details

Returns details about the assigned Data Quota for an endpoint. * `status`: this indicates the current status of the quota and may contain the following values:     - `ACTIVE`: the endpoint can currently connect and has quota left     - `EXHAUSTED`: the endpoint has exceeded the quota volume, if it still can use data service depends on the action chosen to be performed on exhaustion     - `EXPIRED`: the quota has expired; the endpoint is denied from using data services (until new quota is added) * `volume`: returns the volume left on this quota in MB * `expiry_date`: timestamp when this quota will expire and the endpoint will definitely be denied from using further data services (regardless if the quota volume was used up or not) * `peak_throughput`: The maximum bandwidth in octets per second after the endpoint has been throttled. * `action_on_exhaustion`: returns the behaviour defined to be applied when quota volume is used up (exhausted).     - `Throttle`: bandwidth will be throttle to the defined peak throughput until quota expires     - `Block`: data service will be instantly blocked once volume used up, regardless if the expiry date is already reached or not * `auto_refill`: 0 (false) / 1 (true), refill the quota with the last added volume on a daily basis 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**crate::models::EndpointQuota**](EndpointQuota.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_quota_data_by_endpoint_id_post

> endpoint_quota_data_by_endpoint_id_post(endpoint_id, endpoint_quota)
Set Data Quota

At any time, a new data quota can be set for an endpoint. At an initial state when no data quota is set, the endpoint will be denied from using data services. To top-up the data volume you need to retrieve the currently remaining volume, increase it by the top-up volume and set it as the new quota volume.  The following parameters can be configured: * `status` - The status of the quota (mandatory):   - 1: `ACTIVE`   - 2: `EXHAUSTED`   - 3: `EXPIRED` * `volume`: The volume left on this quota in MB * `expiry_date`: Timestamp when this quota will expire and the endpoint will definitely be denied from using further data services (mandatory) * `auto_refill`: Wether the quota shall be refilled on a daily basis (defaults to disabled):   - 0: `disabled`   - 1: `enabled` * `threshold_percentage`: The percentage of remaining quota at which the system should generate a `threshold reached` event * `action_on_exhaustion`: The behaviour of the system after the quota is exhausted:   - id: ID of the action on quota exhaustion (mandatory)     - 1: `Block`     - 2: `Throttle`   - peak_throughput: The maximum bandwidth in octets per second after the endpoint has been throttled. (mandatory)   Allowed values are 64000, 128000, 256000, 384000. (will not take any effect on `action_on_exhaustion` \"Block\")  #### Events The system generates a \"Quota Used Up\" Event in case the data quota is completely depleted. The endpoint will be blocked from further consumption of data. The quota object will be included in the details of the event. Example events can be found in the Events of an Endpoint section.  #### Notes  The endpoint can instantly use data services after the API call to this entrypoint is successfully made. Any timestamp with a future date can be set, this allows to create data packages (e.g. for 1 hour, 24 hour, 7 days or any other timeframe) as required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**endpoint_quota** | [**EndpointQuota**](EndpointQuota.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_quota_sms_by_endpoint_id_get

> serde_json::Value endpoint_quota_sms_by_endpoint_id_get(endpoint_id)
Show SMS Quota details

Returns details about the assigned SMS Quota for an endpoint.  You can retrieve the current SMS quota status for an endpoint with the following API call. It will return following properties: * `status`: this indicates the status of the quota and may contain the following values:   - `ACTIVE`: the endpoint has quota left and can use SMS services   - `EXHAUSTED`: the endpoint has exceeded the quota volume; the endpoint is denied from using SMS services (until new quota is added)   - `EXPIRED`: the quota has expired; the endpoint is denied from using SMS services (until new quota is added) * `volume`: the amount of SMS left on this quota * `expiry_date`: timestamp when this quota will expire and the endpoint will definitely be denied from using further SMS services (regardless if the quota volume was used up or not) * `threshold_percentage`: optional threshold in percentage indicating when a \"Threshold Reached\" event shall be sent. * `action_on_exhaustion`: returns the behaviour defined to be applied when quota volume is used up (exhausted).     - `Throttle`: bandwidth will be throttle to the defined peak throughput until quota expires     - `Block`: data service will be instantly blocked once volume used up, regardless if the expiry date is already reached or not     - `auto_refill`: 0 (false) / 1 (true), refill the quota with the last added volume on a daily basis 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_quota_sms_by_endpoint_id_post

> endpoint_quota_sms_by_endpoint_id_post(endpoint_id, sms_quota)
Set SMS Quota

At any time, a new SMS quota can be set for an endpoint. At an initial state when no SMS quota is set, the endpoint will be denied from using SMS services. To top-up the SMS volume one need to retrieve the currently left volume, increase it by the top-up volume and set it as the new quota volume.  The following parameters can be configured: * `status` - The status of the quota (mandatory):   - 1: `ACTIVE`   - 2: `EXHAUSTED`   - 3: `EXPIRED` * `volume`: The volume left on this quota in MB * `expiry_date`: Timestamp when this quota will expire and the endpoint will definitely be denied from using further data services (mandatory) * `auto_refill`: Wether the quota shall be refilled on a daily basis (defaults to disabled):   - 0: `disabled`   - 1: `enabled` * `threshold_percentage`: The percentage of remaining quota at which the system should generate a `threshold reached` event * `action_on_exhaustion`: The behaviour of the system after the quota is exhausted:   - id: ID of the action on quota exhaustion (mandatory)     - 1: `Block`     - 2: `Throttle` (will not take any effect on SMS quota)   - peak_throughput: The maximum bandwidth in octets per second after the endpoint has been throttled.   Allowed values are 64000, 128000, 256000, 384000. (will not take any effect on SMS quota)  #### Notes  The endpoint can instantly use the SMS service after the API call is successfully made. Any timestamp with a future date can be set, this allows to create SMS packages (e.g. for 1 hour, 24 hour, 7 days or any other timeframe) as required 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**sms_quota** | [**SmsQuota**](SmsQuota.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_sms_by_endpoint_id_and_sms_id_delete

> endpoint_sms_by_endpoint_id_and_sms_id_delete(endpoint_id, sms_id)
Cancel SMS

Cancel SMS that is buffered for endpoint and not yet delivered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**sms_id** | **f32** | The numeric ID of an SMS | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_sms_by_endpoint_id_and_sms_id_get

> crate::models::GetdetailsofSmSresponse endpoint_sms_by_endpoint_id_and_sms_id_get(endpoint_id, sms_id)
SMS details

Returns details about an Endpoint SMS by SMS ID.  A description of the SMS statuses is as follows:  - `1` DELIVERY ATTEMPT PENDING - `2` IN PROGRESS - `3` BUFFERED - `4` DELIVERED - `5` FAILED - `6` EXPIRED - `7` CANCELED 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**sms_id** | **f32** | The numeric ID of an SMS | [required] |

### Return type

[**crate::models::GetdetailsofSmSresponse**](GetdetailsofSMSresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_sms_by_id_get

> Vec<crate::models::ListofSmSresponse> endpoint_sms_by_id_get(endpoint_id)
List sent and received SMS

Returns the list of SMS sent and received by this endpoint.  A description of the SMS statuses is as follows:  - `1` DELIVERY ATTEMPT PENDING - `2` IN PROGRESS - `3` BUFFERED - `4` DELIVERED - `5` FAILED - `6` EXPIRED - `7` CANCELED 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**Vec<crate::models::ListofSmSresponse>**](ListofSMSresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_sms_by_id_post

> endpoint_sms_by_id_post(endpoint_id, submit_mt_sm_srequest)
Send SMS to an Endpoint

Submit MT-SMS to specified endpoint.  You can provide following fields with this request:  * `payload` (String, required) - Message text to be sent, UTF-8 encoded * `source_address` (String, optional) - Source address of SMS: MSISDN, short code or alphanumeric String * `source_address_type` (Object optional) - Specify type of source address. Should contain an `id` property with a value of either:   - `145` - International   - `161` - National   - `208` - Alphanumeric * `expiry_date` (Date, optional) - Expiry date to retain the message until for successful sending. * `udh` (String, optional) - User Data Header encoded has hex-String. Concatenation of multiple SMS messages is done via this value. * `dcs` (Integer, optional) - Data Coding Scheme  #####  Concatenated SMSs  To concatenate SMS messages using `udh` properties, __multiple API calls__ should be made to this entrypoint.  More information on UDH properties can be found in the [3GPP 23.040 specification](https://portal.3gpp.org/desktopmodules/Specifications/SpecificationDetails.aspx?specificationId=747) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**submit_mt_sm_srequest** | [**SubmitMtSmSrequest**](SubmitMtSmSrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_stats_by_id_get

> crate::models::RetrieveEndpointStatisticsresponse endpoint_stats_by_id_get(endpoint_id)
Endpoint Usage and Costs Statistics

Retrieve usage and costs statistics for current/last month/hour. Data traffic costs can only be retrieved for organisations without inclusive volume. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |

### Return type

[**crate::models::RetrieveEndpointStatisticsresponse**](RetrieveEndpointStatisticsresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_stats_daily_by_id_get

> serde_json::Value endpoint_stats_daily_by_id_get(endpoint_id, start_date, end_date)
Endpoint Usage and Costs Statistics per day

Retrieve usage and costs statistics accumulated per days. The statistics for the current month will be returned by default when no date parameters are provided.  The `start_date` and `end_date` query parameters can be provided to filter the returned results by date range. When a `start_date` is provided without an `end_date`, the statistics from the `start_date` until the end of the selected month will be returned.  Data traffic costs can only be retrieved for organisations without inclusive volume. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
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


## endpoint_status_get

> Vec<crate::models::Status> endpoint_status_get()
List Endpoint Statuses

Returns a list of available Endpoint Statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Status>**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connectivity_info_by_endpoint_id

> crate::models::RetrieveConnectivityInformationresponse get_connectivity_info_by_endpoint_id(endpoint_id, subscriber, ussd)
Connectivity Info of an Endpoint

Returns the connectivity information for the specified endpoint by ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**subscriber** | Option<**bool**> | If true, the Subscriber connectivity info is retrieved. Default is true, i.e. subscriber=true |  |
**ussd** | Option<**bool**> | If true, the USSD connectivity info is retrieved. Default is false, i.e. ussd=false |  |

### Return type

[**crate::models::RetrieveConnectivityInformationresponse**](RetrieveConnectivityInformationresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoints

> Vec<crate::models::Endpoint> get_endpoints(q, sort, page, per_page)
List Endpoints

Returns the list of endpoints, filtered, sorted and paged according to query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Expects comma separated list of filtering criteria out of the following fields:  * `status` * `service_profile` * `tariff_profile` * `last_updated` * `created` * `name` * `tags` * `ip_address` * `imei` * `sim_status`  |  |
**sort** | Option<**String**> | Sort properties in a comma separated list from the following fields:  * `status` * `service_profile` * `tariff_profile` * `last_updated` * `created` * `name` * `tags` * `id` * `ip_address` * `imei`  |  |
**page** | Option<**i32**> | Current page number |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |

### Return type

[**Vec<crate::models::Endpoint>**](Endpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_endpoint_connectivity_by_id

> update_endpoint_connectivity_by_id(endpoint_id, update_endpoint_connectivity_by_id_request)
Reset Endpoint Connectivity

Dispatch a message that causes either a `Cancel Location` or `Delete PDP Context` or both sent to the endpoint. The return of the call does not yet mean the event has been sent towards the device.  Master organisation types can reset any endpoints inside their organisation hierarchy, enterprises may only reset connectivity of own endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **i32** | The numeric ID of an Endpoint | [required] |
**update_endpoint_connectivity_by_id_request** | Option<[**UpdateEndpointConnectivityByIdRequest**](UpdateEndpointConnectivityByIdRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

