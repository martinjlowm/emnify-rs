# GetActiveOrganisationInclusiveVolumeResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**volume** | Option<**f32**> | The volume in MB | [optional]
**rate** | Option<**f32**> |  | [optional]
**pooled** | Option<**bool**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**end_date** | Option<**String**> | End date will be omitted in the response, if it has been set to null. This means the inclusive volume will run infinitely.  | [optional]
**currency** | Option<[**crate::models::GetActiveOrganisationInclusiveVolumeResponseInnerCurrency**](GetActiveOrganisationInclusiveVolumeResponse_inner_currency.md)> |  | [optional]
**tariff** | Option<[**crate::models::GetActiveOrganisationInclusiveVolumeResponseInnerTariff**](GetActiveOrganisationInclusiveVolumeResponse_inner_tariff.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


