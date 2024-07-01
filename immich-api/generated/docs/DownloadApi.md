# \DownloadApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_archive**](DownloadApi.md#download_archive) | **POST** /download/archive | 
[**get_download_info**](DownloadApi.md#get_download_info) | **POST** /download/info | 



## download_archive

> std::path::PathBuf download_archive(asset_ids_dto, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_ids_dto** | [**AssetIdsDto**](AssetIdsDto.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_info

> models::DownloadResponseDto get_download_info(download_info_dto, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_info_dto** | [**DownloadInfoDto**](DownloadInfoDto.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**models::DownloadResponseDto**](DownloadResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

