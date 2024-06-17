# \AuditApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audit_deletes**](AuditApi.md#get_audit_deletes) | **GET** /audit/deletes | 



## get_audit_deletes

> models::AuditDeletesResponseDto get_audit_deletes(after, entity_type, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | **String** |  | [required] |
**entity_type** | [**EntityType**](.md) |  | [required] |
**user_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::AuditDeletesResponseDto**](AuditDeletesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

