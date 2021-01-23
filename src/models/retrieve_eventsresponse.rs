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
pub struct RetrieveEventsresponse {
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<serde_json::Value>,
    #[serde(rename = "event_source", skip_serializing_if = "Option::is_none")]
    pub event_source: Option<serde_json::Value>,
    #[serde(rename = "event_severity", skip_serializing_if = "Option::is_none")]
    pub event_severity: Option<serde_json::Value>,
    #[serde(rename = "organisation", skip_serializing_if = "Option::is_none")]
    pub organisation: Option<serde_json::Value>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<serde_json::Value>,
    #[serde(rename = "sim", skip_serializing_if = "Option::is_none")]
    pub sim: Option<serde_json::Value>,
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<serde_json::Value>,
}

impl RetrieveEventsresponse {
    pub fn new() -> RetrieveEventsresponse {
        RetrieveEventsresponse {
            timestamp: None,
            alert: None,
            description: None,
            id: None,
            event_type: None,
            event_source: None,
            event_severity: None,
            organisation: None,
            endpoint: None,
            sim: None,
            imsi: None,
        }
    }
}


