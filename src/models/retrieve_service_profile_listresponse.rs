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
pub struct RetrieveServiceProfileListresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "used_count", skip_serializing_if = "Option::is_none")]
    pub used_count: Option<String>,
    #[serde(rename = "allowed_3g", skip_serializing_if = "Option::is_none")]
    pub allowed_3g: Option<bool>,
    #[serde(rename = "allowed_4g", skip_serializing_if = "Option::is_none")]
    pub allowed_4g: Option<bool>,
    #[serde(rename = "allowed_nb_iot", skip_serializing_if = "Option::is_none")]
    pub allowed_nb_iot: Option<bool>,
    #[serde(rename = "apply_sms_quota", skip_serializing_if = "Option::is_none")]
    pub apply_sms_quota: Option<bool>,
    #[serde(rename = "apply_data_quota", skip_serializing_if = "Option::is_none")]
    pub apply_data_quota: Option<bool>,
}

impl RetrieveServiceProfileListresponse {
    pub fn new() -> RetrieveServiceProfileListresponse {
        RetrieveServiceProfileListresponse {
            id: None,
            name: None,
            description: None,
            used_count: None,
            allowed_3g: None,
            allowed_4g: None,
            allowed_nb_iot: None,
            apply_sms_quota: None,
            apply_data_quota: None,
        }
    }
}


