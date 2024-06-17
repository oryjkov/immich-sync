# \FacesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_faces**](FacesApi.md#get_faces) | **GET** /faces | 
[**reassign_faces_by_id**](FacesApi.md#reassign_faces_by_id) | **PUT** /faces/{id} | 



## get_faces

> Vec<models::AssetFaceResponseDto> get_faces(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AssetFaceResponseDto>**](AssetFaceResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reassign_faces_by_id

> models::PersonResponseDto reassign_faces_by_id(id, face_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**face_dto** | [**FaceDto**](FaceDto.md) |  | [required] |

### Return type

[**models::PersonResponseDto**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

