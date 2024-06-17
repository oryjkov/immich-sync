# \UsersAdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_admin**](UsersAdminApi.md#create_user_admin) | **POST** /admin/users | 
[**delete_user_admin**](UsersAdminApi.md#delete_user_admin) | **DELETE** /admin/users/{id} | 
[**get_user_admin**](UsersAdminApi.md#get_user_admin) | **GET** /admin/users/{id} | 
[**get_user_preferences_admin**](UsersAdminApi.md#get_user_preferences_admin) | **GET** /admin/users/{id}/preferences | 
[**restore_user_admin**](UsersAdminApi.md#restore_user_admin) | **POST** /admin/users/{id}/restore | 
[**search_users_admin**](UsersAdminApi.md#search_users_admin) | **GET** /admin/users | 
[**update_user_admin**](UsersAdminApi.md#update_user_admin) | **PUT** /admin/users/{id} | 
[**update_user_preferences_admin**](UsersAdminApi.md#update_user_preferences_admin) | **PUT** /admin/users/{id}/preferences | 



## create_user_admin

> models::UserAdminResponseDto create_user_admin(user_admin_create_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_admin_create_dto** | [**UserAdminCreateDto**](UserAdminCreateDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_admin

> models::UserAdminResponseDto delete_user_admin(id, user_admin_delete_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_admin_delete_dto** | [**UserAdminDeleteDto**](UserAdminDeleteDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_admin

> models::UserAdminResponseDto get_user_admin(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_preferences_admin

> models::UserPreferencesResponseDto get_user_preferences_admin(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_user_admin

> models::UserAdminResponseDto restore_user_admin(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_admin

> Vec<models::UserAdminResponseDto> search_users_admin(with_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**bool**> |  |  |

### Return type

[**Vec<models::UserAdminResponseDto>**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_admin

> models::UserAdminResponseDto update_user_admin(id, user_admin_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_admin_update_dto** | [**UserAdminUpdateDto**](UserAdminUpdateDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_preferences_admin

> models::UserPreferencesResponseDto update_user_preferences_admin(id, user_preferences_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_preferences_update_dto** | [**UserPreferencesUpdateDto**](UserPreferencesUpdateDto.md) |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

