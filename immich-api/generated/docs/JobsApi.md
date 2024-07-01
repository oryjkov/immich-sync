# \JobsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_jobs_status**](JobsApi.md#get_all_jobs_status) | **GET** /jobs | 
[**send_job_command**](JobsApi.md#send_job_command) | **PUT** /jobs/{id} | 



## get_all_jobs_status

> models::AllJobStatusResponseDto get_all_jobs_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllJobStatusResponseDto**](AllJobStatusResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_job_command

> models::JobStatusDto send_job_command(id, job_command_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**JobName**](.md) |  | [required] |
**job_command_dto** | [**JobCommandDto**](JobCommandDto.md) |  | [required] |

### Return type

[**models::JobStatusDto**](JobStatusDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

