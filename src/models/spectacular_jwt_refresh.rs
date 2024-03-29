/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpectacularJwtRefresh : Refresh Access token.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectacularJwtRefresh {
    /// Your access token
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Access token expires in seconds
    #[serde(rename = "access_expires", skip_serializing_if = "Option::is_none")]
    pub access_expires: Option<i32>,
}

impl SpectacularJwtRefresh {
    /// Refresh Access token.
    pub fn new() -> SpectacularJwtRefresh {
        SpectacularJwtRefresh {
            access: None,
            access_expires: None,
        }
    }
}


