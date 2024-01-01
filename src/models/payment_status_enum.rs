/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentStatusEnum : * `INIT` - Initiated. Payment has been initiated. * `ERRE` - ExecutionError. We experienced error on payment execution. * `ERRS` - StatusError. We experienced error retrieving payment status. Try again. * `ACCC` - AcceptedSettlementCompleted. Settlement on the creditor's account has been completed * `ACCP` - AcceptedCustomerProfile. Preceding check of technical validation was successful. Customer profile check was successful * `ACSC` - AcceptedSettlementCompleted. Settlement on the debtor’s account has been completed * `ACSP` - AcceptedSettlementInProcess. All preceding checks such as technical validation and customer profile were successful and therefore the payment initiation has been accepted for execution * `ACTC` - AcceptedTechnicalValidation. Authentication and syntactical and semantical validation are successful * `ACWC` - AcceptedWithChange. Instruction is accepted but a change will be made, such as date or remittance not sent * `ACWP` - AcceptedWithoutPosting. Payment instruction included in the credit transfer is accepted without being posted to the creditor customer’s account * `RCVD` - Received. Payment initiation has been received by the receiving agent * `PDNG` - Pending. Payment initiation or individual transaction included in the payment initiation is pending. Further checks and status update will be performed * `RJCT` - Rejected. Payment initiation or individual transaction included in the payment initiation has been rejected. * `CANC` - Cancelled. Payment initiation has been cancelled before execution * `ACFC` - AcceptedFundsChecked. Pre-ceeding check of technical validation and customer profile was successful and an automatic funds check was positive * `PATC` - PartiallyAcceptedTechnicalCorrect. The payment initiation needs multiple authentications, where some but not yet all have been performed * `PART` - PartiallyAccepted. A number of transactions have been accepted, whereas another number of transactions have not yet achieved 'accepted' status

/// * `INIT` - Initiated. Payment has been initiated. * `ERRE` - ExecutionError. We experienced error on payment execution. * `ERRS` - StatusError. We experienced error retrieving payment status. Try again. * `ACCC` - AcceptedSettlementCompleted. Settlement on the creditor's account has been completed * `ACCP` - AcceptedCustomerProfile. Preceding check of technical validation was successful. Customer profile check was successful * `ACSC` - AcceptedSettlementCompleted. Settlement on the debtor’s account has been completed * `ACSP` - AcceptedSettlementInProcess. All preceding checks such as technical validation and customer profile were successful and therefore the payment initiation has been accepted for execution * `ACTC` - AcceptedTechnicalValidation. Authentication and syntactical and semantical validation are successful * `ACWC` - AcceptedWithChange. Instruction is accepted but a change will be made, such as date or remittance not sent * `ACWP` - AcceptedWithoutPosting. Payment instruction included in the credit transfer is accepted without being posted to the creditor customer’s account * `RCVD` - Received. Payment initiation has been received by the receiving agent * `PDNG` - Pending. Payment initiation or individual transaction included in the payment initiation is pending. Further checks and status update will be performed * `RJCT` - Rejected. Payment initiation or individual transaction included in the payment initiation has been rejected. * `CANC` - Cancelled. Payment initiation has been cancelled before execution * `ACFC` - AcceptedFundsChecked. Pre-ceeding check of technical validation and customer profile was successful and an automatic funds check was positive * `PATC` - PartiallyAcceptedTechnicalCorrect. The payment initiation needs multiple authentications, where some but not yet all have been performed * `PART` - PartiallyAccepted. A number of transactions have been accepted, whereas another number of transactions have not yet achieved 'accepted' status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatusEnum {
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "ERRE")]
    Erre,
    #[serde(rename = "ERRS")]
    Errs,
    #[serde(rename = "ACCC")]
    Accc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACSC")]
    Acsc,
    #[serde(rename = "ACSP")]
    Acsp,
    #[serde(rename = "ACTC")]
    Actc,
    #[serde(rename = "ACWC")]
    Acwc,
    #[serde(rename = "ACWP")]
    Acwp,
    #[serde(rename = "RCVD")]
    Rcvd,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "ACFC")]
    Acfc,
    #[serde(rename = "PATC")]
    Patc,
    #[serde(rename = "PART")]
    Part,

}

impl ToString for PaymentStatusEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Init => String::from("INIT"),
            Self::Erre => String::from("ERRE"),
            Self::Errs => String::from("ERRS"),
            Self::Accc => String::from("ACCC"),
            Self::Accp => String::from("ACCP"),
            Self::Acsc => String::from("ACSC"),
            Self::Acsp => String::from("ACSP"),
            Self::Actc => String::from("ACTC"),
            Self::Acwc => String::from("ACWC"),
            Self::Acwp => String::from("ACWP"),
            Self::Rcvd => String::from("RCVD"),
            Self::Pdng => String::from("PDNG"),
            Self::Rjct => String::from("RJCT"),
            Self::Canc => String::from("CANC"),
            Self::Acfc => String::from("ACFC"),
            Self::Patc => String::from("PATC"),
            Self::Part => String::from("PART"),
        }
    }
}

impl Default for PaymentStatusEnum {
    fn default() -> PaymentStatusEnum {
        Self::Init
    }
}




