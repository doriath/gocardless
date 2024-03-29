/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstructedAmount : InstructedAmountSerializer.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructedAmount {
    /// Instructed amount
    #[serde(rename = "amount")]
    pub amount: String,
    /// Instructed amount currency
    #[serde(rename = "currency")]
    pub currency: String,
}

impl InstructedAmount {
    /// InstructedAmountSerializer.
    pub fn new(amount: String, currency: String) -> InstructedAmount {
        InstructedAmount {
            amount,
            currency,
        }
    }
}


