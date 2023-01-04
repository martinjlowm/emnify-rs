# ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | URL of your Webhook / RestAPI endpoint | 
**method** | Option<**String**> | HTTP Method to use. Options are `POST` (default), `PUT` and `PATCH` | [optional]
**headers** | Option<**Vec<String>**> | Array containing additional header tuples. Expected Format is `[\"header1:value1, \"header2:value2\", ...]`. Headers are masked in GET request. | [optional]
**size** | Option<**i32**> | Number specifying of request should contain `1` message or an array of messages of the specified batch size. Default is `3000`. Valid sizes are between 1 and 10000. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


