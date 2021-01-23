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
pub struct RetrieveOperatorBlacklistresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "tapcode", skip_serializing_if = "Option::is_none")]
    pub tapcode: Option<Vec<serde_json::Value>>,
    #[serde(rename = "mnc", skip_serializing_if = "Option::is_none")]
    pub mnc: Option<Vec<serde_json::Value>>,
}

impl RetrieveOperatorBlacklistresponse {
    pub fn new() -> RetrieveOperatorBlacklistresponse {
        RetrieveOperatorBlacklistresponse {
            id: None,
            name: None,
            country: None,
            tapcode: None,
            mnc: None,
        }
    }
}


