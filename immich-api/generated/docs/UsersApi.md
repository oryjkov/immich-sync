# \UsersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile_image**](UsersApi.md#create_profile_image) | **POST** /users/profile-image | 
[**delete_profile_image**](UsersApi.md#delete_profile_image) | **DELETE** /users/profile-image | 
[**get_my_preferences**](UsersApi.md#get_my_preferences) | **GET** /users/me/preferences | 
[**get_my_user**](UsersApi.md#get_my_user) | **GET** /users/me | 
[**get_profile_image**](UsersApi.md#get_profile_image) | **GET** /users/{id}/profile-image | 
[**get_user**](UsersApi.md#get_user) | **GET** /users/{id} | 
[**search_users**](UsersApi.md#search_users) | **GET** /users | 
[**update_my_preferences**](UsersApi.md#update_my_preferences) | **PUT** /users/me/preferences | 
[**update_my_user**](UsersApi.md#update_my_user) | **PUT** /users/me | 



## create_profile_image

> models::CreateProfileImageResponseDto create_profile_image(file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**models::CreateProfileImageResponseDto**](CreateProfileImageResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_profile_image

> delete_profile_image()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_preferences

> models::UserPreferencesResponseDto get_my_preferences()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user

> models::UserAdminResponseDto get_my_user()


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


## get_profile_image

> std::path::PathBuf get_profile_image(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::UserResponseDto get_user(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<models::UserResponseDto> search_users()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserResponseDto>**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_preferences

> models::UserPreferencesResponseDto update_my_preferences(user_preferences_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_preferences_update_dto** | [**UserPreferencesUpdateDto**](UserPreferencesUpdateDto.md) |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_user

> models::UserAdminResponseDto update_my_user(user_update_me_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_update_me_dto** | [**UserUpdateMeDto**](UserUpdateMeDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

