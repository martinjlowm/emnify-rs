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
pub struct RetrieveConnectivityInformationresponse {
    #[serde(rename = "ussd_info", skip_serializing_if = "Option::is_none")]
    pub ussd_info: Option<serde_json::Value>,
    #[serde(rename = "subscriber_info", skip_serializing_if = "Option::is_none")]
    pub subscriber_info: Option<serde_json::Value>,
    #[serde(rename = "request_timestamp", skip_serializing_if = "Option::is_none")]
    pub request_timestamp: Option<String>,
    #[serde(rename = "reply_timestamp", skip_serializing_if = "Option::is_none")]
    pub reply_timestamp: Option<String>,
}

impl RetrieveConnectivityInformationresponse {
    pub fn new() -> RetrieveConnectivityInformationresponse {
        RetrieveConnectivityInformationresponse {
            ussd_info: None,
            subscriber_info: None,
            request_timestamp: None,
            reply_timestamp: None,
        }
    }
}


