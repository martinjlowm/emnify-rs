# ListDataStreamerV2s200ResponseDestinationCredentials

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | **String** | Datadog region. Options are `US` (default), `US3`, `EU` and `US1FED` | 
**stream_name** | **String** | Name of your Kinesis Data Stream | 
**role_arn** | **String** | Role ARN of IAM role with write permission for your S3 bucket | 
**bucket_name** | **String** | Name of your S3 bucket | 
**flush_size** | Option<**i32**> | Maximal number of entries in a single file. Default is `10000`. Valid flush sizes are between 10 and 100000. | [optional]
**rotate_interval** | Option<**i32**> | Maximal time in milliseconds between file writes. Default is `10000` ms. Valid rotation intervals are between 10000 ms and 900000 ms. | [optional]
**url** | **String** | URL of your Webhook / RestAPI endpoint | 
**method** | Option<**String**> | HTTP Method to use. Options are `POST` (default), `PUT` and `PATCH` | [optional]
**headers** | Option<**Vec<String>**> | Array containing additional header tuples. Expected Format is `[\"header1:value1, \"header2:value2\", ...]`. Headers are masked in GET request. | [optional]
**size** | Option<**i32**> | Number specifying of request should contain `1` message or an array of messages of the specified batch size. Default is `3000`. Valid sizes are between 1 and 10000. | [optional]
**project_id** | **String** | Project ID reported by KeenIO | 
**write_key** | **String** | Write key of your KeenIO Project. | 
**collection_name** | **String** | Collection Name that you want to stream your data to. | 
**api_key** | **String** | Datadog API Key | 
**connection_string** | **String** | Connection string with Access Key to your Event Hub. | 
**cps_topic** | **String** | Topic name of PubSub topic | 
**cps_project** | **String** | Project ID of the topic project. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


