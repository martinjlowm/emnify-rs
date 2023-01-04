# OrganisationTariffPlanByOrgIdGet200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f32**> | ID of the tariff plan assignment | [optional]
**organisation_id** | Option<**String**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**expiry_date** | Option<**String**> |  | [optional]
**sim_activated_rate** | Option<**f32**> |  | [optional]
**sim_activated_idle_rate** | Option<**f32**> |  | [optional]
**sim_suspended_rate** | Option<**f32**> |  | [optional]
**sim_suspended_active_rate** | Option<**f32**> |  | [optional]
**currency_id** | Option<**String**> |  | [optional]
**federation_allowed** | Option<**bool**> | Custom federation_allowed configuration for the organisation. If set to null, the default configuration from the tariff plan will be applied.  | [optional]
**service_level** | Option<[**crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerServiceLevel**](OrganisationTariffPlanByOrgIdGet_200_response_inner_service_level.md)> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**applied_rate** | Option<**f32**> |  | [optional]
**price_model** | Option<**String**> |  | [optional]
**tariff_plan** | Option<[**crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan**](OrganisationTariffPlanByOrgIdGet_200_response_inner_tariff_plan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


