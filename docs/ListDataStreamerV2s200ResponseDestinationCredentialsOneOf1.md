# ListDataStreamerV2s200ResponseDestinationCredentialsOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | **String** | AWS Region of your Kinesis Data Stream | 
**bucket_name** | **String** | Name of your S3 bucket | 
**role_arn** | **String** | Role ARN of IAM role with write permission for your S3 bucket | 
**flush_size** | Option<**i32**> | Maximal number of entries in a single file. Default is `10000`. Valid flush sizes are between 10 and 100000. | [optional]
**rotate_interval** | Option<**i32**> | Maximal time in milliseconds between file writes. Default is `10000` ms. Valid rotation intervals are between 10000 ms and 900000 ms. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


