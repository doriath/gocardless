/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSchema {
    /// iban
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// bban
    #[serde(rename = "bban", skip_serializing_if = "Option::is_none")]
    pub bban: Option<String>,
    /// pan
    #[serde(rename = "pan", skip_serializing_if = "Option::is_none")]
    pub pan: Option<String>,
    /// maskedPan
    #[serde(rename = "maskedPan", skip_serializing_if = "Option::is_none")]
    pub masked_pan: Option<String>,
    /// msisdn
    #[serde(rename = "msisdn", skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<String>,
    /// currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl AccountSchema {
    pub fn new() -> AccountSchema {
        AccountSchema {
            iban: None,
            bban: None,
            pan: None,
            masked_pan: None,
            msisdn: None,
            currency: None,
        }
    }
}


