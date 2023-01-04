# \PasswordManagementAndActivationApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_activation_post**](PasswordManagementAndActivationApi.md#user_activation_post) | **POST** /api/v1/user/activation | Activate User
[**user_activation_resend_post**](PasswordManagementAndActivationApi.md#user_activation_resend_post) | **POST** /api/v1/user/activation_resend | Resend User Activation E-mail
[**user_password_patch**](PasswordManagementAndActivationApi.md#user_password_patch) | **PATCH** /api/v1/user/password | Change Password



## user_activation_post

> user_activation_post(account_activationrequest)
Activate User

This service activates the user account and sets a password. The activation key is sent via email.  In this request, the following details should be provided: * `activation_token` (String required) - the activation token sent to a user via email * `password` (String required)  __Note:__ This is a public (unauthenticated) service which ignores auth tokens 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_activationrequest** | [**AccountActivationrequest**](AccountActivationrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_activation_resend_post

> user_activation_resend_post(re_send_activation_mailrequest)
Resend User Activation E-mail

This service re-sends activation mail to the user.  As this endpoint is open to the public (no authentication token necessary), it requires instead the google reCAPTCHA token to ensure that no robot is performing the request.  Moreover there is a time limit on how often a given user may be issued with a new Activation Mail.  User has to provide:  * `username` (String required) * `g-recaptcha-response` (String required) - auto-generated from a form using Google reCAPTCHA   > According to the [reCAPTCHA documentation](https://developers.google.com/recaptcha/docs/display) a \"Site Key\" is necessary. If you want to implement this feature in your client, please contact EMnify support to obtain this \"Site Key\" for your domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**re_send_activation_mailrequest** | [**ReSendActivationMailrequest**](ReSendActivationMailrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_password_patch

> user_password_patch(change_passwordrequest)
Change Password

Password change service.  Allows to change the password for the currently authenticated user.  User has to provide:  * `old_password` (String required) * `new_password`  (String required)  #### Notes  * The client application should invalidate the authentication token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_passwordrequest** | [**ChangePasswordrequest**](ChangePasswordrequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

