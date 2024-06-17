# \AuthenticationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_password**](AuthenticationApi.md#change_password) | **POST** /auth/change-password | 
[**login**](AuthenticationApi.md#login) | **POST** /auth/login | 
[**logout**](AuthenticationApi.md#logout) | **POST** /auth/logout | 
[**sign_up_admin**](AuthenticationApi.md#sign_up_admin) | **POST** /auth/admin-sign-up | 
[**validate_access_token**](AuthenticationApi.md#validate_access_token) | **POST** /auth/validateToken | 



## change_password

> models::UserAdminResponseDto change_password(change_password_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_dto** | [**ChangePasswordDto**](ChangePasswordDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> models::LoginResponseDto login(login_credential_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_credential_dto** | [**LoginCredentialDto**](LoginCredentialDto.md) |  | [required] |

### Return type

[**models::LoginResponseDto**](LoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> models::LogoutResponseDto logout()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LogoutResponseDto**](LogoutResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_up_admin

> models::UserAdminResponseDto sign_up_admin(sign_up_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_dto** | [**SignUpDto**](SignUpDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_access_token

> models::ValidateAccessTokenResponseDto validate_access_token()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ValidateAccessTokenResponseDto**](ValidateAccessTokenResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

