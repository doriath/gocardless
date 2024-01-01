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
pub struct CurrencyExchangeSchema {
    /// sourceCurrency
    #[serde(rename = "sourceCurrency", skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// exchangeRate
    #[serde(rename = "exchangeRate", skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    /// unitCurrency
    #[serde(rename = "unitCurrency", skip_serializing_if = "Option::is_none")]
    pub unit_currency: Option<String>,
    /// targetCurrency
    #[serde(rename = "targetCurrency", skip_serializing_if = "Option::is_none")]
    pub target_currency: Option<String>,
    /// quotationDate
    #[serde(rename = "quotationDate", skip_serializing_if = "Option::is_none")]
    pub quotation_date: Option<String>,
    /// contractIdentification
    #[serde(rename = "contractIdentification", skip_serializing_if = "Option::is_none")]
    pub contract_identification: Option<String>,
}

impl CurrencyExchangeSchema {
    pub fn new() -> CurrencyExchangeSchema {
        CurrencyExchangeSchema {
            source_currency: None,
            exchange_rate: None,
            unit_currency: None,
            target_currency: None,
            quotation_date: None,
            contract_identification: None,
        }
    }
}


