# \SearchApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assets_by_city**](SearchApi.md#get_assets_by_city) | **GET** /search/cities | 
[**get_explore_data**](SearchApi.md#get_explore_data) | **GET** /search/explore | 
[**get_search_suggestions**](SearchApi.md#get_search_suggestions) | **GET** /search/suggestions | 
[**search_metadata**](SearchApi.md#search_metadata) | **POST** /search/metadata | 
[**search_person**](SearchApi.md#search_person) | **GET** /search/person | 
[**search_places**](SearchApi.md#search_places) | **GET** /search/places | 
[**search_smart**](SearchApi.md#search_smart) | **POST** /search/smart | 



## get_assets_by_city

> Vec<models::AssetResponseDto> get_assets_by_city()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_explore_data

> Vec<models::SearchExploreResponseDto> get_explore_data()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SearchExploreResponseDto>**](SearchExploreResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggestions

> Vec<String> get_search_suggestions(r#type, country, make, model, state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**SearchSuggestionType**](.md) |  | [required] |
**country** | Option<**String**> |  |  |
**make** | Option<**String**> |  |  |
**model** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_metadata

> models::SearchResponseDto search_metadata(metadata_search_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_search_dto** | [**MetadataSearchDto**](MetadataSearchDto.md) |  | [required] |

### Return type

[**models::SearchResponseDto**](SearchResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_person

> Vec<models::PersonResponseDto> search_person(name, with_hidden)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**with_hidden** | Option<**bool**> |  |  |

### Return type

[**Vec<models::PersonResponseDto>**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_places

> Vec<models::PlacesResponseDto> search_places(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**Vec<models::PlacesResponseDto>**](PlacesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_smart

> models::SearchResponseDto search_smart(smart_search_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_search_dto** | [**SmartSearchDto**](SmartSearchDto.md) |  | [required] |

### Return type

[**models::SearchResponseDto**](SearchResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

