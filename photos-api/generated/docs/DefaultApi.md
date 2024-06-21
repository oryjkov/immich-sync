# \DefaultApi

All URIs are relative to *https://photoslibrary.googleapis.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_media_item**](DefaultApi.md#download_media_item) | **GET** /mediaItems/{mediaItemId}/download | Download media item
[**get_media_item**](DefaultApi.md#get_media_item) | **GET** /mediaItems/{mediaItemId} | Get media item metadata
[**list_albums**](DefaultApi.md#list_albums) | **GET** /albums | List albums
[**list_shared_albums**](DefaultApi.md#list_shared_albums) | **GET** /sharedAlbums | List shared albums



## download_media_item

> std::path::PathBuf download_media_item(media_item_id)
Download media item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_item_id** | **String** | ID of the media item to download. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_item

> models::MediaItem get_media_item(media_item_id)
Get media item metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_item_id** | **String** | ID of the media item to retrieve metadata for. | [required] |

### Return type

[**models::MediaItem**](MediaItem.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_albums

> models::ListAlbumsResponse list_albums(page_size, page_token)
List albums

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Maximum number of albums to return. |  |[default to 20]
**page_token** | Option<**String**> | Token to retrieve the next page of results. |  |

### Return type

[**models::ListAlbumsResponse**](ListAlbumsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_shared_albums

> models::ListSharedAlbumsResponse list_shared_albums(page_size, page_token)
List shared albums

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Maximum number of shared albums to return. |  |[default to 20]
**page_token** | Option<**String**> | Token to retrieve the next page of results. |  |

### Return type

[**models::ListSharedAlbumsResponse**](ListSharedAlbumsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

