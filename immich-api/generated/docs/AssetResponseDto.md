# AssetResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checksum** | **String** | base64 encoded sha1 hash | 
**device_asset_id** | **String** |  | 
**device_id** | **String** |  | 
**duplicate_id** | Option<**String**> |  | [optional]
**duration** | **String** |  | 
**exif_info** | Option<[**models::ExifResponseDto**](ExifResponseDto.md)> |  | [optional]
**file_created_at** | **String** |  | 
**file_modified_at** | **String** |  | 
**has_metadata** | **bool** |  | 
**id** | **String** |  | 
**is_archived** | **bool** |  | 
**is_favorite** | **bool** |  | 
**is_offline** | **bool** |  | 
**is_trashed** | **bool** |  | 
**library_id** | Option<**String**> | This property was deprecated in v1.106.0 | [optional]
**live_photo_video_id** | Option<**String**> |  | [optional]
**local_date_time** | **String** |  | 
**original_file_name** | **String** |  | 
**original_mime_type** | Option<**String**> |  | [optional]
**original_path** | **String** |  | 
**owner** | Option<[**models::UserResponseDto**](UserResponseDto.md)> |  | [optional]
**owner_id** | **String** |  | 
**people** | Option<[**Vec<models::PersonWithFacesResponseDto>**](PersonWithFacesResponseDto.md)> |  | [optional]
**resized** | **bool** |  | 
**smart_info** | Option<[**models::SmartInfoResponseDto**](SmartInfoResponseDto.md)> |  | [optional]
**stack** | Option<[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)> |  | [optional]
**stack_count** | Option<**i32**> |  | [optional]
**stack_parent_id** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<models::TagResponseDto>**](TagResponseDto.md)> |  | [optional]
**thumbhash** | Option<**String**> |  | 
**r#type** | [**models::AssetTypeEnum**](AssetTypeEnum.md) |  | 
**unassigned_faces** | Option<[**Vec<models::AssetFaceWithoutPersonResponseDto>**](AssetFaceWithoutPersonResponseDto.md)> |  | [optional]
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


