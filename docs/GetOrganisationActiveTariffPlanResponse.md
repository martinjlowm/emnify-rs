# GetOrganisationActiveTariffPlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organisation_id** | Option<**f32**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**expiry_date** | Option<**String**> | the end date of the tariff_plan, or null if it is open end  | [optional]
**sim_activated_rate** | Option<**f32**> |  | [optional]
**sim_activated_idle_rate** | Option<**f32**> |  | [optional]
**sim_suspended_rate** | Option<**f32**> |  | [optional]
**sim_suspended_active_rate** | Option<**f32**> |  | [optional]
**sim_activated_amount** | Option<**f32**> | the number of activated SIMs this month  | [optional]
**applied_price** | Option<[**crate::models::GetOrganisationActiveTariffPlanResponseAppliedPrice**](GetOrganisationActiveTariffPlanResponse_applied_price.md)> |  | [optional]
**tariff_plan** | Option<[**crate::models::GetOrganisationActiveTariffPlanResponseTariffPlan**](GetOrganisationActiveTariffPlanResponse_tariff_plan.md)> |  | [optional]
**tariff_plan_runtime** | Option<[**crate::models::GetOrganisationActiveTariffPlanResponseTariffPlanRuntime**](GetOrganisationActiveTariffPlanResponse_tariff_plan_runtime.md)> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**applied_rate** | Option<**f32**> |  | [optional]
**id** | Option<**f32**> |  | [optional]
**service_level** | Option<[**crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanServiceLevel**](OrganisationTariffPlanByOrgIdGet_200_response_inner_tariff_plan_service_level.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


