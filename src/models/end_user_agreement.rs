/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EndUserAgreement : Represents an end-user agreement.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndUserAgreement {
    /// The ID of this End User Agreement, used to refer to this end user agreement in other API calls.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The date & time at which the end user agreement was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
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
    /// The date & time at which the end user accepted the agreement.
    #[serde(rename = "accepted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accepted: Option<Option<String>>,
}

impl EndUserAgreement {
    /// Represents an end-user agreement.
    pub fn new(institution_id: String) -> EndUserAgreement {
        EndUserAgreement {
            id: None,
            created: None,
            institution_id,
            max_historical_days: None,
            access_valid_for_days: None,
            access_scope: None,
            accepted: None,
        }
    }
}

