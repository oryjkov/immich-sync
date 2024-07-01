# \SystemConfigApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config**](SystemConfigApi.md#get_config) | **GET** /system-config | 
[**get_config_defaults**](SystemConfigApi.md#get_config_defaults) | **GET** /system-config/defaults | 
[**get_storage_template_options**](SystemConfigApi.md#get_storage_template_options) | **GET** /system-config/storage-template-options | 
[**update_config**](SystemConfigApi.md#update_config) | **PUT** /system-config | 



## get_config

> models::SystemConfigDto get_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemConfigDto**](SystemConfigDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config_defaults

> models::SystemConfigDto get_config_defaults()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemConfigDto**](SystemConfigDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_template_options

> models::SystemConfigTemplateStorageOptionDto get_storage_template_options()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemConfigTemplateStorageOptionDto**](SystemConfigTemplateStorageOptionDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config

> models::SystemConfigDto update_config(system_config_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_config_dto** | [**SystemConfigDto**](SystemConfigDto.md) |  | [required] |

### Return type

[**models::SystemConfigDto**](SystemConfigDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

