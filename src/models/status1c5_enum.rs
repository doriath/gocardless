/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status1c5Enum {
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "LN")]
    Ln,
    #[serde(rename = "RJ")]
    Rj,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "SU")]
    Su,
    #[serde(rename = "EX")]
    Ex,
    #[serde(rename = "GC")]
    Gc,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "SA")]
    Sa,

}

impl ToString for Status1c5Enum {
    fn to_string(&self) -> String {
        match self {
            Self::Cr => String::from("CR"),
            Self::Id => String::from("ID"),
            Self::Ln => String::from("LN"),
            Self::Rj => String::from("RJ"),
            Self::Er => String::from("ER"),
            Self::Su => String::from("SU"),
            Self::Ex => String::from("EX"),
            Self::Gc => String::from("GC"),
            Self::Ua => String::from("UA"),
            Self::Ga => String::from("GA"),
            Self::Sa => String::from("SA"),
        }
    }
}

impl Default for Status1c5Enum {
    fn default() -> Status1c5Enum {
        Self::Cr
    }
}




