/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpectacularJwtObtain : Obtain new JWT pair.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectacularJwtObtain {
    /// Your access token
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Access token expires in seconds
    #[serde(rename = "access_expires", skip_serializing_if = "Option::is_none")]
    pub access_expires: Option<i32>,
    /// Your refresh token
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    /// Refresh token expires in seconds
    #[serde(rename = "refresh_expires", skip_serializing_if = "Option::is_none")]
    pub refresh_expires: Option<i32>,
}

impl SpectacularJwtObtain {
    /// Obtain new JWT pair.
    pub fn new() -> SpectacularJwtObtain {
        SpectacularJwtObtain {
            access: None,
            access_expires: None,
            refresh: None,
            refresh_expires: None,
        }
    }
}


