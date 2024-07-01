# SharedLinkEditDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_download** | Option<**bool**> |  | [optional]
**allow_upload** | Option<**bool**> |  | [optional]
**change_expiry_time** | Option<**bool**> | Few clients cannot send null to set the expiryTime to never. Setting this flag and not sending expiryAt is considered as null instead. Clients that can send null values can ignore this. | [optional]
**description** | Option<**String**> |  | [optional]
**expires_at** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**show_metadata** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


