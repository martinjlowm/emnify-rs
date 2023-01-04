# GetMonthlyOrganisationStatsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_month** | Option<[**crate::models::OrganisationTrafficStatisticsMonthObject**](Organisation_traffic_statistics_month_object.md)> |  | [optional]
**last_month** | Option<[**crate::models::OrganisationTrafficStatisticsMonthObject**](Organisation_traffic_statistics_month_object.md)> |  | [optional]
**sim** | Option<[**crate::models::GetMonthlyOrganisationStatsResponseSim**](Get_monthly_organisation_stats_response_sim.md)> |  | [optional]
**service_profiles** | Option<**f32**> | Amount of service profiles | [optional]
**tariff_profiles** | Option<**f32**> | Amount of service profiles | [optional]
**users** | Option<**f32**> | Amount of users | [optional]
**active_chargeable_sims** | Option<**f32**> | Amount of active SIMs that will be charged within the current month. This field is omitted if there are no chargeable SIMs.  | [optional]
**hosting_fees** | Option<**f32**> | Total of SIM hosting fees for all `active_chargeable_sims`. This field is omitted if there are no chargeable SIMs. | [optional]
**inclusive_volume** | Option<[**crate::models::GetMonthlyOrganisationStatsResponseInclusiveVolume**](Get_monthly_organisation_stats_response_inclusive_volume.md)> |  | [optional]
**prepaid_balance** | Option<[**crate::models::GetMonthlyOrganisationStatsResponsePrepaidBalance**](Get_monthly_organisation_stats_response_prepaid_balance.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


