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
pub struct UpdateTariffRequest {
    /// ID of the new tariff
    #[serde(rename = "id")]
    pub id: f32,
}

impl UpdateTariffRequest {
    pub fn new(id: f32) -> UpdateTariffRequest {
        UpdateTariffRequest {
            id,
        }
    }
}


