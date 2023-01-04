# \UserManagementApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_by_id_delete**](UserManagementApi.md#user_by_id_delete) | **DELETE** /api/v1/user/{user_id} | Delete User by ID
[**user_by_id_delete_v2**](UserManagementApi.md#user_by_id_delete_v2) | **DELETE** /api/v2/user/{user_id} | Delete User by ID
[**user_by_id_get**](UserManagementApi.md#user_by_id_get) | **GET** /api/v1/user/{user_id} | Get User by ID or Username
[**user_by_id_patch**](UserManagementApi.md#user_by_id_patch) | **PATCH** /api/v1/user/{user_id} | Update User by ID
[**user_by_id_v2_create_support_token**](UserManagementApi.md#user_by_id_v2_create_support_token) | **POST** /api/v2/user/{user_id}/support_token | Create support access token for given user
[**user_event_page_per_page_sort_by_user_id_and_q_get**](UserManagementApi.md#user_event_page_per_page_sort_by_user_id_and_q_get) | **GET** /api/v1/user/{user_id}/event | List User Events
[**user_per_page_sort_by_q_and_page_get**](UserManagementApi.md#user_per_page_sort_by_q_and_page_get) | **GET** /api/v1/user | List User Accounts
[**user_per_page_sort_by_q_and_page_post**](UserManagementApi.md#user_per_page_sort_by_q_and_page_post) | **POST** /api/v1/user | Create User
[**user_role_by_id_and_role_id_delete**](UserManagementApi.md#user_role_by_id_and_role_id_delete) | **DELETE** /api/v1/user/{user_id}/role/{role_id} | Delete User Role
[**user_role_by_id_and_role_id_put**](UserManagementApi.md#user_role_by_id_and_role_id_put) | **PUT** /api/v1/user/{user_id}/role/{role_id} | Assign Role to User
[**user_role_get**](UserManagementApi.md#user_role_get) | **GET** /api/v1/user/role | List User Roles
[**user_role_permission_by_id_get**](UserManagementApi.md#user_role_permission_by_id_get) | **GET** /api/v1/user/{user_id}/role/permission | List User Role Permissions
[**user_status_get**](UserManagementApi.md#user_status_get) | **GET** /api/v1/user/status | List User Statuses



## user_by_id_delete

> user_by_id_delete(user_id)
Delete User by ID

**DEPRECATED** Please use DELETE `/api/v2/user/:id` instead **Notes** * A user can be deleted, if belonging to the same organisation as the requesting user,  or to an organisation which is a direct child of the requesting user's organisation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_by_id_delete_v2

> user_by_id_delete_v2(user_id)
Delete User by ID

**Notes** A user can be deleted, if: * belonging to the same organisation as the requesting user * belonging to an organisation which is a direct child of the requesting user's organisation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_by_id_get

> crate::models::RetrievetheUserresponse user_by_id_get(user_id)
Get User by ID or Username

Get a specific user by ID  provided the user is within this requesting user's organisation or the organisation's immediate child organisations.  `id` may be one of:  * The numeric ID of the user, e.g. \"123\". This is the top-level `id` object returned by each item in `GET /api/v1/user` * Username (email), e.g. \"exampleuser@org.de\". This is the top-level `username` object returned by each item in `GET /api/v1/user` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |

### Return type

[**crate::models::RetrievetheUserresponse**](RetrievetheUserresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_by_id_patch

> user_by_id_patch(user_id, update_userrequest)
Update User by ID

Provided fields * `username` (String optional) - has to be user's email * `name` (String optional) * `status` (Object optional) * `organisation` (Object optional) - must be the current organisation or not to be provided  #### Notes  * Password is not provided. Separate calls provide password management. * The organisation is not modifiable. A user can be updated, if belonging to the same organisation as the requesting user, or to an organisation which is a direct child of the requesting user's organisation. * Status can be changed between ACTIVE (id: 1) and SUSPENDED (id: 2), and from ACTIVATION_PENDING (id: 0) to SUSPENDED.  > Modifying the username invalidates account and triggers the activation procedure. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**update_userrequest** | [**UpdateUserrequest**](UpdateUserrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_by_id_v2_create_support_token

> crate::models::UserByIdV2CreateSupportTokenRequest1 user_by_id_v2_create_support_token(user_id, user_by_id_v2_create_support_token_request)
Create support access token for given user

Create a bearer token that can be used to acces the portal as the target user. Additionally admin or observer role can be selected. The role will not be applied to the user, only to the token.  The access can be made through cannel-partner hierarchies as well. An event will be generated on the accessed organisation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**user_by_id_v2_create_support_token_request** | Option<[**UserByIdV2CreateSupportTokenRequest**](UserByIdV2CreateSupportTokenRequest.md)> |  |  |

### Return type

[**crate::models::UserByIdV2CreateSupportTokenRequest1**](UserByIdV2CreateSupportTokenRequest_1.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_event_page_per_page_sort_by_user_id_and_q_get

> Vec<crate::models::RetrieveEventsresponse4> user_event_page_per_page_sort_by_user_id_and_q_get(user_id, page, per_page, sort, q)
List User Events

Returns the list of events, filtered, sorted and paged according to query parameters.  Only an administrator or observer may be allowed to see events of other users, provided they belong to an organisation he/she has access to.  Any user can retrieve their own events at `/api/v1/user/my/event`.  **CAUTION** This API endpoint deviates from the specified conventions and may not return the same HTTP Codes as the higher layer call (`/api/v1/user/{user_id|my}`). In case the requested `{user_id}` does not exist or is not accessible for the user, **HTTP 200** will be returned with empty **[]** as long as the provided `{user_id}` is a number and all parameters are valid. Please take that into consideration when building automation on top of the error behaviour of this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**page** | Option<**i32**> | Current page number |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |
**sort** | Option<**String**> | Sort properties according to a comma separated list of accepted fields. Valid fields are:  * `id` - (**event id**) * `timestamp` - (**event timestamp**) * `source` - (**event source**) * `severity` - (**event severity**) * `alert` - (**alert status**) * `organisation` - (**organisation name**) * `user` - (**user id**) * `endpoint` - (**endpoint name**) * `tags` - (**endpoint tags**) * `ip_address` - (**endpoint ip_address**) * `iccid` - (**sim iccid**) * `imsi` - (**sim imsi**) * `type` - (**event type**)  |  |
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Multiple filters must be in the format of a comma separated list of the following fields:  * `type` (**event_type**, numerical) * `source` (**event_type**, numerical, e.g. 0 = Network), 1 = Policy Control, 2 = API) * `severity` (**event_severity**, numerical, e.g. 0 = Info, 1 = Warn), 2 = Critical) * `alert` (boolean, e.g. true, false) * `description` (**event description**, string) * `organisation` (**organisation name**, string) * `user` (**user name**, string) * `endpoint` (**endpoint name**, string) * `tags` (**endpoint tags**, string) * `ip_address` (**endpoint IP address**, valid IPv4/IPv6 address) * `imei` (**endpoint imei**, numerical string) * `iccid` (**sim iccid**, numerical string) * `imsi` (**sim imsi**, numerical string) * `from` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with until!__) * `until` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, __only valid with from!__) * `timestamp` (**date**, format `YYYY-MM-DDTHH:mm:ssZ`, for querying events of 1 day, deprecated in future)  |  |

### Return type

[**Vec<crate::models::RetrieveEventsresponse4>**](RetrieveEventsresponse4.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_per_page_sort_by_q_and_page_get

> Vec<crate::models::User> user_per_page_sort_by_q_and_page_get(page, q, per_page, sort)
List User Accounts

Retrieves the collection of user accounts, filtered, sorted and paged accourding to query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f32**> | Current page number |  |
**q** | Option<**String**> | Filter parameter in `<filter>:<value>` format. Multiple filters must be in the format of a comma separated list of the following fields:  * `id` * `status` * `name` * `username` * `organisation`  |  |
**per_page** | Option<**i32**> | Paging parameters defining the number of items per page |  |
**sort** | Option<**String**> | Sort properties according to comma separated list out of the allowed fields  * `id` * `status` * `name` * `username` * `organisation` * `created`  |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_per_page_sort_by_q_and_page_post

> user_per_page_sort_by_q_and_page_post(create_userrequest)
Create User

Creates a user which is *not active and has no password assigned*. The URL to get the user details is provided as Location Header in the response.  #### ACTIVATION  Upon creation, the user account undergoes an activation process (see services below ) in which she receives an email with activation link. Following this link the user is asked to set the password and upon successful completion of this process, the account becomes active and operational.  Provided fields:  * `username` (String required) - has to be the email of this user * `name` (String required) * `organisation` (Object optional) -  **may be provided** by regular user, but **is required** for master user. * `roles` (List of Objects optional) - List of one or more role Ids to be assigned at once. If missing, a default role is assigned  #### Notes  * Password is not provided. Separate calls provide password management. * When the organisation of the new user is not specified in the request, it is inherited from the user creating the account.  A regular user is allowed to specify **only organisations which are direct children** of his/her own organisation, or his/her own organisation. * The status field is not user editable at account creation time - the default imposed by server is ACTIVATION_PENDING. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_userrequest** | [**CreateUserrequest**](CreateUserrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_role_by_id_and_role_id_delete

> user_role_by_id_and_role_id_delete(user_id, role_id)
Delete User Role

Release a Role from association with this user.  Note that a Role can only be removed, if it is not the last role of this user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**role_id** | **f32** | Role ID to be assigned | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_role_by_id_and_role_id_put

> user_role_by_id_and_role_id_put(user_id, role_id)
Assign Role to User

Role is assigned to this user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |
**role_id** | **f32** | Role ID to be assigned | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_role_get

> Vec<crate::models::UserRoleGet200ResponseInner> user_role_get()
List User Roles

Retrieves the collection of available user roles

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserRoleGet200ResponseInner>**](UserRoleGet_200_response_inner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_role_permission_by_id_get

> serde_json::Value user_role_permission_by_id_get(user_id)
List User Role Permissions

Role permissions available to this user.  Only an administrator or observer is allowed to see the role permissions of other users, provided they belong to an organisation he/she has access to.<br/>  Any user can also retrieve one's own role permissions at: `/api/v1/user/my/role/permission`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **f32** | User ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_status_get

> Vec<crate::models::RetrieveAvailableUserStatusesresponse> user_status_get()
List User Statuses

Provides the list of available user status (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableUserStatusesresponse>**](RetrieveAvailableUserStatusesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

