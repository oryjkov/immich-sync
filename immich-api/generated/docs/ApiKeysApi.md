# \ApiKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /api-keys | 
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api-keys/{id} | 
[**get_api_key**](ApiKeysApi.md#get_api_key) | **GET** /api-keys/{id} | 
[**get_api_keys**](ApiKeysApi.md#get_api_keys) | **GET** /api-keys | 
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PUT** /api-keys/{id} | 



## create_api_key

> models::ApiKeyCreateResponseDto create_api_key(api_key_create_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_create_dto** | [**ApiKeyCreateDto**](ApiKeyCreateDto.md) |  | [required] |

### Return type

[**models::ApiKeyCreateResponseDto**](APIKeyCreateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> models::ApiKeyResponseDto get_api_key(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ApiKeyResponseDto**](APIKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> Vec<models::ApiKeyResponseDto> get_api_keys()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiKeyResponseDto>**](APIKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> models::ApiKeyResponseDto update_api_key(id, api_key_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_key_update_dto** | [**ApiKeyUpdateDto**](ApiKeyUpdateDto.md) |  | [required] |

### Return type

[**models::ApiKeyResponseDto**](APIKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

