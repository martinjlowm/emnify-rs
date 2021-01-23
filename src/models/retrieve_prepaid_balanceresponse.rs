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
pub struct RetrievePrepaidBalanceresponse {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}

impl RetrievePrepaidBalanceresponse {
    pub fn new() -> RetrievePrepaidBalanceresponse {
        RetrievePrepaidBalanceresponse {
            amount: None,
            currency: None,
            last_updated: None,
            expiry_date: None,
        }
    }
}


