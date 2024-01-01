/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Account : The representation of a bank account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// The ID of this Account, used to refer to this account in other API calls.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The date & time at which the account object was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The date & time at which the account object was last accessed.
    #[serde(rename = "last_accessed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<Option<String>>,
    /// The Account IBAN
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// The ASPSP associated with this account.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::AccountStatusEnum>,
    /// The name of the account owner.
    #[serde(rename = "owner_name", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
}

impl Account {
    /// The representation of a bank account.
    pub fn new() -> Account {
        Account {
            id: None,
            created: None,
            last_accessed: None,
            iban: None,
            institution_id: None,
            status: None,
            owner_name: None,
        }
    }
}


