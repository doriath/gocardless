/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnduserAcceptanceDetailsRequest : Represents end-user details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnduserAcceptanceDetailsRequest {
    #[serde(rename = "user_agent")]
    pub user_agent: String,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl EnduserAcceptanceDetailsRequest {
    /// Represents end-user details.
    pub fn new(user_agent: String, ip_address: String) -> EnduserAcceptanceDetailsRequest {
        EnduserAcceptanceDetailsRequest {
            user_agent,
            ip_address,
        }
    }
}

