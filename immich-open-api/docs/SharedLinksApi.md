# \SharedLinksApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_shared_link_assets**](SharedLinksApi.md#add_shared_link_assets) | **PUT** /shared-links/{id}/assets | 
[**create_shared_link**](SharedLinksApi.md#create_shared_link) | **POST** /shared-links | 
[**get_all_shared_links**](SharedLinksApi.md#get_all_shared_links) | **GET** /shared-links | 
[**get_my_shared_link**](SharedLinksApi.md#get_my_shared_link) | **GET** /shared-links/me | 
[**get_shared_link_by_id**](SharedLinksApi.md#get_shared_link_by_id) | **GET** /shared-links/{id} | 
[**remove_shared_link**](SharedLinksApi.md#remove_shared_link) | **DELETE** /shared-links/{id} | 
[**remove_shared_link_assets**](SharedLinksApi.md#remove_shared_link_assets) | **DELETE** /shared-links/{id}/assets | 
[**update_shared_link**](SharedLinksApi.md#update_shared_link) | **PATCH** /shared-links/{id} | 



## add_shared_link_assets

> Vec<models::AssetIdsResponseDto> add_shared_link_assets(id, asset_ids_dto, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_ids_dto** | [**AssetIdsDto**](AssetIdsDto.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**Vec<models::AssetIdsResponseDto>**](AssetIdsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_shared_link

> models::SharedLinkResponseDto create_shared_link(shared_link_create_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shared_link_create_dto** | [**SharedLinkCreateDto**](SharedLinkCreateDto.md) |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_shared_links

> Vec<models::SharedLinkResponseDto> get_all_shared_links()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SharedLinkResponseDto>**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_shared_link

> models::SharedLinkResponseDto get_my_shared_link(key, password, token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**token** | Option<**String**> |  |  |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_link_by_id

> models::SharedLinkResponseDto get_shared_link_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_shared_link

> remove_shared_link(id)


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


## remove_shared_link_assets

> Vec<models::AssetIdsResponseDto> remove_shared_link_assets(id, asset_ids_dto, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_ids_dto** | [**AssetIdsDto**](AssetIdsDto.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**Vec<models::AssetIdsResponseDto>**](AssetIdsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shared_link

> models::SharedLinkResponseDto update_shared_link(id, shared_link_edit_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**shared_link_edit_dto** | [**SharedLinkEditDto**](SharedLinkEditDto.md) |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

