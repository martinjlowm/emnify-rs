/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.2.26
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2007 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The volume in MB
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    #[serde(rename = "pooled", skip_serializing_if = "Option::is_none")]
    pub pooled: Option<bool>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date will be omitted in the response, if it has been set to null. This means the inclusive volume will run infinitely. 
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::models::ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveCurrency>,
    #[serde(rename = "tariff", skip_serializing_if = "Option::is_none")]
    pub tariff: Option<crate::models::ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveTariff>,
}

impl InlineResponse2007 {
    pub fn new() -> InlineResponse2007 {
        InlineResponse2007 {
            id: None,
            volume: None,
            rate: None,
            pooled: None,
            start_date: None,
            end_date: None,
            currency: None,
            tariff: None,
        }
    }
}


