# \SystemMetadataApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_admin_onboarding**](SystemMetadataApi.md#get_admin_onboarding) | **GET** /system-metadata/admin-onboarding | 
[**get_reverse_geocoding_state**](SystemMetadataApi.md#get_reverse_geocoding_state) | **GET** /system-metadata/reverse-geocoding-state | 
[**update_admin_onboarding**](SystemMetadataApi.md#update_admin_onboarding) | **POST** /system-metadata/admin-onboarding | 



## get_admin_onboarding

> models::AdminOnboardingUpdateDto get_admin_onboarding()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AdminOnboardingUpdateDto**](AdminOnboardingUpdateDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reverse_geocoding_state

> models::ReverseGeocodingStateResponseDto get_reverse_geocoding_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReverseGeocodingStateResponseDto**](ReverseGeocodingStateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_admin_onboarding

> update_admin_onboarding(admin_onboarding_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_onboarding_update_dto** | [**AdminOnboardingUpdateDto**](AdminOnboardingUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

