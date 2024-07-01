# \FileReportsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fix_audit_files**](FileReportsApi.md#fix_audit_files) | **POST** /reports/fix | 
[**get_audit_files**](FileReportsApi.md#get_audit_files) | **GET** /reports | 
[**get_file_checksums**](FileReportsApi.md#get_file_checksums) | **POST** /reports/checksum | 



## fix_audit_files

> fix_audit_files(file_report_fix_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_report_fix_dto** | [**FileReportFixDto**](FileReportFixDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_files

> models::FileReportDto get_audit_files()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FileReportDto**](FileReportDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_checksums

> Vec<models::FileChecksumResponseDto> get_file_checksums(file_checksum_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_checksum_dto** | [**FileChecksumDto**](FileChecksumDto.md) |  | [required] |

### Return type

[**Vec<models::FileChecksumResponseDto>**](FileChecksumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

