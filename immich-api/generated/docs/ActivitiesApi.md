# \ActivitiesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_activity**](ActivitiesApi.md#create_activity) | **POST** /activities | 
[**delete_activity**](ActivitiesApi.md#delete_activity) | **DELETE** /activities/{id} | 
[**get_activities**](ActivitiesApi.md#get_activities) | **GET** /activities | 
[**get_activity_statistics**](ActivitiesApi.md#get_activity_statistics) | **GET** /activities/statistics | 



## create_activity

> models::ActivityResponseDto create_activity(activity_create_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_create_dto** | [**ActivityCreateDto**](ActivityCreateDto.md) |  | [required] |

### Return type

[**models::ActivityResponseDto**](ActivityResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity

> delete_activity(id)


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


## get_activities

> Vec<models::ActivityResponseDto> get_activities(album_id, asset_id, level, r#type, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **uuid::Uuid** |  | [required] |
**asset_id** | Option<**uuid::Uuid**> |  |  |
**level** | Option<[**ReactionLevel**](.md)> |  |  |
**r#type** | Option<[**ReactionType**](.md)> |  |  |
**user_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::ActivityResponseDto>**](ActivityResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_statistics

> models::ActivityStatisticsResponseDto get_activity_statistics(album_id, asset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **uuid::Uuid** |  | [required] |
**asset_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::ActivityStatisticsResponseDto**](ActivityStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

