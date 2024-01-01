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
pub struct OwnerAddressStructuredSchema {
    /// streetName
    #[serde(rename = "streetName", skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,
    /// buildingNumber
    #[serde(rename = "buildingNumber", skip_serializing_if = "Option::is_none")]
    pub building_number: Option<String>,
    /// townName
    #[serde(rename = "townName", skip_serializing_if = "Option::is_none")]
    pub town_name: Option<String>,
    /// postCode
    #[serde(rename = "postCode", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl OwnerAddressStructuredSchema {
    pub fn new() -> OwnerAddressStructuredSchema {
        OwnerAddressStructuredSchema {
            street_name: None,
            building_number: None,
            town_name: None,
            post_code: None,
            country: None,
        }
    }
}


