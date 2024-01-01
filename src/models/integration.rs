/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Integration : Represents an Integration.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "bic", skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "transaction_total_days", skip_serializing_if = "Option::is_none")]
    pub transaction_total_days: Option<String>,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "logo")]
    pub logo: String,
}

impl Integration {
    /// Represents an Integration.
    pub fn new(id: String, name: String, countries: Vec<String>, logo: String) -> Integration {
        Integration {
            id,
            name,
            bic: None,
            transaction_total_days: None,
            countries,
            logo,
        }
    }
}


