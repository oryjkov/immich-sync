# \SyncApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_delta_sync**](SyncApi.md#get_delta_sync) | **POST** /sync/delta-sync | 
[**get_full_sync_for_user**](SyncApi.md#get_full_sync_for_user) | **POST** /sync/full-sync | 



## get_delta_sync

> models::AssetDeltaSyncResponseDto get_delta_sync(asset_delta_sync_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_delta_sync_dto** | [**AssetDeltaSyncDto**](AssetDeltaSyncDto.md) |  | [required] |

### Return type

[**models::AssetDeltaSyncResponseDto**](AssetDeltaSyncResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_full_sync_for_user

> Vec<models::AssetResponseDto> get_full_sync_for_user(asset_full_sync_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_full_sync_dto** | [**AssetFullSyncDto**](AssetFullSyncDto.md) |  | [required] |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

