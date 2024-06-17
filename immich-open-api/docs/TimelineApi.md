# \TimelineApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_bucket**](TimelineApi.md#get_time_bucket) | **GET** /timeline/bucket | 
[**get_time_buckets**](TimelineApi.md#get_time_buckets) | **GET** /timeline/buckets | 



## get_time_bucket

> Vec<models::AssetResponseDto> get_time_bucket(size, time_bucket, album_id, is_archived, is_favorite, is_trashed, key, order, person_id, user_id, with_partners, with_stacked)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**size** | [**TimeBucketSize**](.md) |  | [required] |
**time_bucket** | **String** |  | [required] |
**album_id** | Option<**uuid::Uuid**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_trashed** | Option<**bool**> |  |  |
**key** | Option<**String**> |  |  |
**order** | Option<[**AssetOrder**](.md)> |  |  |
**person_id** | Option<**uuid::Uuid**> |  |  |
**user_id** | Option<**uuid::Uuid**> |  |  |
**with_partners** | Option<**bool**> |  |  |
**with_stacked** | Option<**bool**> |  |  |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_buckets

> Vec<models::TimeBucketResponseDto> get_time_buckets(size, album_id, is_archived, is_favorite, is_trashed, key, order, person_id, user_id, with_partners, with_stacked)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**size** | [**TimeBucketSize**](.md) |  | [required] |
**album_id** | Option<**uuid::Uuid**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_trashed** | Option<**bool**> |  |  |
**key** | Option<**String**> |  |  |
**order** | Option<[**AssetOrder**](.md)> |  |  |
**person_id** | Option<**uuid::Uuid**> |  |  |
**user_id** | Option<**uuid::Uuid**> |  |  |
**with_partners** | Option<**bool**> |  |  |
**with_stacked** | Option<**bool**> |  |  |

### Return type

[**Vec<models::TimeBucketResponseDto>**](TimeBucketResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

