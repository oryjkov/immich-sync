# \AssetsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_bulk_upload**](AssetsApi.md#check_bulk_upload) | **POST** /assets/bulk-upload-check | 
[**check_existing_assets**](AssetsApi.md#check_existing_assets) | **POST** /assets/exist | 
[**delete_assets**](AssetsApi.md#delete_assets) | **DELETE** /assets | 
[**download_asset**](AssetsApi.md#download_asset) | **GET** /assets/{id}/original | 
[**get_all_user_assets_by_device_id**](AssetsApi.md#get_all_user_assets_by_device_id) | **GET** /assets/device/{deviceId} | 
[**get_asset_info**](AssetsApi.md#get_asset_info) | **GET** /assets/{id} | 
[**get_asset_statistics**](AssetsApi.md#get_asset_statistics) | **GET** /assets/statistics | 
[**get_memory_lane**](AssetsApi.md#get_memory_lane) | **GET** /assets/memory-lane | 
[**get_random**](AssetsApi.md#get_random) | **GET** /assets/random | 
[**play_asset_video**](AssetsApi.md#play_asset_video) | **GET** /assets/{id}/video/playback | 
[**replace_asset**](AssetsApi.md#replace_asset) | **PUT** /assets/{id}/original | 
[**run_asset_jobs**](AssetsApi.md#run_asset_jobs) | **POST** /assets/jobs | 
[**update_asset**](AssetsApi.md#update_asset) | **PUT** /assets/{id} | 
[**update_assets**](AssetsApi.md#update_assets) | **PUT** /assets | 
[**update_stack_parent**](AssetsApi.md#update_stack_parent) | **PUT** /assets/stack/parent | 
[**upload_asset**](AssetsApi.md#upload_asset) | **POST** /assets | 
[**view_asset**](AssetsApi.md#view_asset) | **GET** /assets/{id}/thumbnail | 



## check_bulk_upload

> models::AssetBulkUploadCheckResponseDto check_bulk_upload(asset_bulk_upload_check_dto)


Checks if assets exist by checksums

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_upload_check_dto** | [**AssetBulkUploadCheckDto**](AssetBulkUploadCheckDto.md) |  | [required] |

### Return type

[**models::AssetBulkUploadCheckResponseDto**](AssetBulkUploadCheckResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_existing_assets

> models::CheckExistingAssetsResponseDto check_existing_assets(check_existing_assets_dto)


Checks if multiple assets exist on the server and returns all existing - used by background backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_existing_assets_dto** | [**CheckExistingAssetsDto**](CheckExistingAssetsDto.md) |  | [required] |

### Return type

[**models::CheckExistingAssetsResponseDto**](CheckExistingAssetsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assets

> delete_assets(asset_bulk_delete_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_delete_dto** | [**AssetBulkDeleteDto**](AssetBulkDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_asset

> std::path::PathBuf download_asset(id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_assets_by_device_id

> Vec<String> get_all_user_assets_by_device_id(device_id)


Get all asset of a device that are in the database, ID only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_info

> models::AssetResponseDto get_asset_info(id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_statistics

> models::AssetStatsResponseDto get_asset_statistics(is_archived, is_favorite, is_trashed)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_trashed** | Option<**bool**> |  |  |

### Return type

[**models::AssetStatsResponseDto**](AssetStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_memory_lane

> Vec<models::MemoryLaneResponseDto> get_memory_lane(day, month)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **i32** |  | [required] |
**month** | **i32** |  | [required] |

### Return type

[**Vec<models::MemoryLaneResponseDto>**](MemoryLaneResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random

> Vec<models::AssetResponseDto> get_random(count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**f64**> |  |  |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## play_asset_video

> std::path::PathBuf play_asset_video(id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_asset

> models::AssetMediaResponseDto replace_asset(id, asset_data, device_asset_id, device_id, file_created_at, file_modified_at, key, duration)


Replace the asset with new file, without changing its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_data** | **std::path::PathBuf** |  | [required] |
**device_asset_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**file_created_at** | **String** |  | [required] |
**file_modified_at** | **String** |  | [required] |
**key** | Option<**String**> |  |  |
**duration** | Option<**String**> |  |  |

### Return type

[**models::AssetMediaResponseDto**](AssetMediaResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_asset_jobs

> run_asset_jobs(asset_jobs_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_jobs_dto** | [**AssetJobsDto**](AssetJobsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_asset

> models::AssetResponseDto update_asset(id, update_asset_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_asset_dto** | [**UpdateAssetDto**](UpdateAssetDto.md) |  | [required] |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_assets

> update_assets(asset_bulk_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_update_dto** | [**AssetBulkUpdateDto**](AssetBulkUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stack_parent

> update_stack_parent(update_stack_parent_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_stack_parent_dto** | [**UpdateStackParentDto**](UpdateStackParentDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_asset

> models::AssetMediaResponseDto upload_asset(asset_data, device_asset_id, device_id, file_created_at, file_modified_at, key, x_immich_checksum, duration, is_archived, is_favorite, is_offline, is_visible, live_photo_video_id, sidecar_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_data** | **std::path::PathBuf** |  | [required] |
**device_asset_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**file_created_at** | **String** |  | [required] |
**file_modified_at** | **String** |  | [required] |
**key** | Option<**String**> |  |  |
**x_immich_checksum** | Option<**String**> | sha1 checksum that can be used for duplicate detection before the file is uploaded |  |
**duration** | Option<**String**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_offline** | Option<**bool**> |  |  |
**is_visible** | Option<**bool**> |  |  |
**live_photo_video_id** | Option<**uuid::Uuid**> |  |  |
**sidecar_data** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::AssetMediaResponseDto**](AssetMediaResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_asset

> std::path::PathBuf view_asset(id, key, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**size** | Option<[**AssetMediaSize**](.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

