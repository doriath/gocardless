/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EndUserAgreementRequest : Represents an end-user agreement.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndUserAgreementRequest {
    /// an Institution ID for this EUA
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// Maximum number of days of transaction data to retrieve.
    #[serde(rename = "max_historical_days", skip_serializing_if = "Option::is_none")]
    pub max_historical_days: Option<i32>,
    /// Number of days from acceptance that the access can be used.
    #[serde(rename = "access_valid_for_days", skip_serializing_if = "Option::is_none")]
    pub access_valid_for_days: Option<i32>,
    /// Array containing one or several values of ['balances', 'details', 'transactions']
    #[serde(rename = "access_scope", skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<Vec<serde_json::Value>>,
}

impl EndUserAgreementRequest {
    /// Represents an end-user agreement.
    pub fn new(institution_id: String) -> EndUserAgreementRequest {
        EndUserAgreementRequest {
            institution_id,
            max_historical_days: None,
            access_valid_for_days: None,
            access_scope: None,
        }
    }
}


