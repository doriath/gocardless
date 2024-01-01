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
pub struct Sct {
    /// Link to initiate authorization with Institution
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Payment ID
    #[serde(rename = "payment_id", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<crate::models::PaymentStatusEnum>,
    #[serde(rename = "payment_product", skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<crate::models::PaymentProductEnum>,
    #[serde(rename = "payment_type", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<crate::models::PaymentTypeEnum>,
    #[serde(rename = "instructed_amount")]
    pub instructed_amount: Box<crate::models::InstructedAmount>,
    #[serde(rename = "creditor_object")]
    pub creditor_object: Box<crate::models::CreditorAccountWrite>,
    /// Redirect URL to your application after payment is done
    #[serde(rename = "redirect", deserialize_with = "Option::deserialize")]
    pub redirect: Option<String>,
    /// Payment description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payment Custom Payment ID
    #[serde(rename = "custom_payment_id", skip_serializing_if = "Option::is_none")]
    pub custom_payment_id: Option<String>,
    /// Payment Execution date (for periodic payments)
    #[serde(rename = "requested_execution_date", skip_serializing_if = "Option::is_none")]
    pub requested_execution_date: Option<String>,
    /// Indicates whether payment should be submitted separately
    #[serde(rename = "submit_payment", skip_serializing_if = "Option::is_none")]
    pub submit_payment: Option<bool>,
}

impl Sct {
    pub fn new(instructed_amount: crate::models::InstructedAmount, creditor_object: crate::models::CreditorAccountWrite, redirect: Option<String>) -> Sct {
        Sct {
            link: None,
            payment_id: None,
            payment_status: None,
            payment_product: None,
            payment_type: None,
            instructed_amount: Box::new(instructed_amount),
            creditor_object: Box::new(creditor_object),
            redirect,
            description: None,
            custom_payment_id: None,
            requested_execution_date: None,
            submit_payment: None,
        }
    }
}


