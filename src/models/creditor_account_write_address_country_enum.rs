/*
 * GoCardless Bank Account Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0 (v2)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditorAccountWriteAddressCountryEnum : * `BE` - Belgien * `BG` - Bulgarien * `DK` - Dänemark * `DE` - Deutschland * `EE` - Estland * `FI` - Finnland * `FR` - Frankreich * `GR` - Griechenland * `IE` - Irland * `IS` - Island * `IT` - Italien * `HR` - Kroatien * `LV` - Lettland * `LI` - Liechtenstein * `LT` - Litauen * `LU` - Luxemburg * `MT` - Malta * `NL` - Niederlande * `NO` - Norwegen * `AT` - Österreich * `PL` - Polen * `PT` - Portugal * `RO` - Rumänien * `SE` - Schweden * `SK` - Slowakei * `SI` - Slowenien * `ES` - Spanien * `CZ` - Tschechien * `HU` - Ungarn * `US` - Vereinigte Staaten von Amerika * `GB` - Vereinigtes Königreich * `CY` - Zypern

/// * `BE` - Belgien * `BG` - Bulgarien * `DK` - Dänemark * `DE` - Deutschland * `EE` - Estland * `FI` - Finnland * `FR` - Frankreich * `GR` - Griechenland * `IE` - Irland * `IS` - Island * `IT` - Italien * `HR` - Kroatien * `LV` - Lettland * `LI` - Liechtenstein * `LT` - Litauen * `LU` - Luxemburg * `MT` - Malta * `NL` - Niederlande * `NO` - Norwegen * `AT` - Österreich * `PL` - Polen * `PT` - Portugal * `RO` - Rumänien * `SE` - Schweden * `SK` - Slowakei * `SI` - Slowenien * `ES` - Spanien * `CZ` - Tschechien * `HU` - Ungarn * `US` - Vereinigte Staaten von Amerika * `GB` - Vereinigtes Königreich * `CY` - Zypern
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditorAccountWriteAddressCountryEnum {
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "CY")]
    Cy,

}

impl ToString for CreditorAccountWriteAddressCountryEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Be => String::from("BE"),
            Self::Bg => String::from("BG"),
            Self::Dk => String::from("DK"),
            Self::De => String::from("DE"),
            Self::Ee => String::from("EE"),
            Self::Fi => String::from("FI"),
            Self::Fr => String::from("FR"),
            Self::Gr => String::from("GR"),
            Self::Ie => String::from("IE"),
            Self::Is => String::from("IS"),
            Self::It => String::from("IT"),
            Self::Hr => String::from("HR"),
            Self::Lv => String::from("LV"),
            Self::Li => String::from("LI"),
            Self::Lt => String::from("LT"),
            Self::Lu => String::from("LU"),
            Self::Mt => String::from("MT"),
            Self::Nl => String::from("NL"),
            Self::No => String::from("NO"),
            Self::At => String::from("AT"),
            Self::Pl => String::from("PL"),
            Self::Pt => String::from("PT"),
            Self::Ro => String::from("RO"),
            Self::Se => String::from("SE"),
            Self::Sk => String::from("SK"),
            Self::Si => String::from("SI"),
            Self::Es => String::from("ES"),
            Self::Cz => String::from("CZ"),
            Self::Hu => String::from("HU"),
            Self::Us => String::from("US"),
            Self::Gb => String::from("GB"),
            Self::Cy => String::from("CY"),
        }
    }
}

impl Default for CreditorAccountWriteAddressCountryEnum {
    fn default() -> CreditorAccountWriteAddressCountryEnum {
        Self::Be
    }
}




