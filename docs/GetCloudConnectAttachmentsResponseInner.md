# GetCloudConnectAttachmentsResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**user_id** | Option<**i32**> |  | [optional]
**creation_date** | Option<**String**> | The date this attachment was created in UTC | [optional]
**accept_attachment_expiry_date** | Option<**String**> | The expiry date of the accept attachment state in UTC. This will only be returned if the breakout is of type `Transit Gateway (type_id: 1)` and in Status `Pending AWS Actvation (status_id: 2)`  | [optional]
**termination_date** | Option<**String**> |  | [optional]
**aws_transit_gateway_attachment_id** | Option<**String**> |  | [optional]
**aws_vpn_connection_id** | Option<**String**> | This is only set when the breakout is a VPN attachment | [optional]
**status** | Option<[**crate::models::CreateMfaKeyResponseStatus**](Create_MFA_Key_Response_status.md)> |  | [optional]
**r#type** | Option<[**crate::models::CreateMfaKeyResponseStatus**](Create_MFA_Key_Response_status.md)> |  | [optional]
**region** | Option<**String**> | The customer region that this attachment belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


