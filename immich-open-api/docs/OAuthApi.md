# \OAuthApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**finish_o_auth**](OAuthApi.md#finish_o_auth) | **POST** /oauth/callback | 
[**link_o_auth_account**](OAuthApi.md#link_o_auth_account) | **POST** /oauth/link | 
[**redirect_o_auth_to_mobile**](OAuthApi.md#redirect_o_auth_to_mobile) | **GET** /oauth/mobile-redirect | 
[**start_o_auth**](OAuthApi.md#start_o_auth) | **POST** /oauth/authorize | 
[**unlink_o_auth_account**](OAuthApi.md#unlink_o_auth_account) | **POST** /oauth/unlink | 



## finish_o_auth

> models::LoginResponseDto finish_o_auth(o_auth_callback_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_callback_dto** | [**OAuthCallbackDto**](OAuthCallbackDto.md) |  | [required] |

### Return type

[**models::LoginResponseDto**](LoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_o_auth_account

> models::UserAdminResponseDto link_o_auth_account(o_auth_callback_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_callback_dto** | [**OAuthCallbackDto**](OAuthCallbackDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redirect_o_auth_to_mobile

> redirect_o_auth_to_mobile()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_o_auth

> models::OAuthAuthorizeResponseDto start_o_auth(o_auth_config_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_config_dto** | [**OAuthConfigDto**](OAuthConfigDto.md) |  | [required] |

### Return type

[**models::OAuthAuthorizeResponseDto**](OAuthAuthorizeResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_o_auth_account

> models::UserAdminResponseDto unlink_o_auth_account()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

