# OrganisationDailyTrafficObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cost** | Option<**String**> | Total cost | [optional]
**currency_id** | Option<**String**> |  | [optional]
**currency** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**traffic_type_id** | Option<**String**> |  | [optional]
**traffic_type** | Option<[**crate::models::OrganisationTrafficStatisticsObjectTrafficType**](Organisation_traffic_statistics_object_traffic_type.md)> |  | [optional]
**volume** | Option<**String**> | Total consumption (`volume_rx` + `volume_tx`) | [optional]
**volume_rx** | Option<**String**> | * For traffic type `5` (`Data`): Downloaded data * For traffic type `6` (`SMS`): SMS MT  | [optional]
**volume_tx** | Option<**String**> | * For traffic type `5` (`Data`): Uploaded data * For traffic type `6` (`SMS`): SMS MO  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


