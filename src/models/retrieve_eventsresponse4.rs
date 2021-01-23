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
pub struct RetrieveEventsresponse4 {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "alert")]
    pub alert: bool,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event_type")]
    pub event_type: serde_json::Value,
    #[serde(rename = "event_source")]
    pub event_source: serde_json::Value,
    #[serde(rename = "event_severity")]
    pub event_severity: serde_json::Value,
    #[serde(rename = "organisation")]
    pub organisation: serde_json::Value,
    #[serde(rename = "user")]
    pub user: serde_json::Value,
}

impl RetrieveEventsresponse4 {
    pub fn new(id: i32, alert: bool, description: String, timestamp: String, event_type: serde_json::Value, event_source: serde_json::Value, event_severity: serde_json::Value, organisation: serde_json::Value, user: serde_json::Value) -> RetrieveEventsresponse4 {
        RetrieveEventsresponse4 {
            id,
            alert,
            description,
            timestamp,
            event_type,
            event_source,
            event_severity,
            organisation,
            user,
        }
    }
}


