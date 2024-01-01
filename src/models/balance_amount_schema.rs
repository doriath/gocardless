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
pub struct BalanceAmountSchema {
    /// amount
    #[serde(rename = "amount")]
    pub amount: String,
    /// currency
    #[serde(rename = "currency")]
    pub currency: String,
}

impl BalanceAmountSchema {
    pub fn new(amount: String, currency: String) -> BalanceAmountSchema {
        BalanceAmountSchema {
            amount,
            currency,
        }
    }
}


