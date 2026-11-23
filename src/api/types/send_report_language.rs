pub use crate::prelude::*;

/// Language of email content for campaign report sending.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SendReportLanguage {
    Fr,
    Es,
    Pt,
    It,
    De,
    En,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SendReportLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fr => serializer.serialize_str("fr"),
            Self::Es => serializer.serialize_str("es"),
            Self::Pt => serializer.serialize_str("pt"),
            Self::It => serializer.serialize_str("it"),
            Self::De => serializer.serialize_str("de"),
            Self::En => serializer.serialize_str("en"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SendReportLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fr" => Ok(Self::Fr),
            "es" => Ok(Self::Es),
            "pt" => Ok(Self::Pt),
            "it" => Ok(Self::It),
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SendReportLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fr => write!(f, "fr"),
            Self::Es => write!(f, "es"),
            Self::Pt => write!(f, "pt"),
            Self::It => write!(f, "it"),
            Self::De => write!(f, "de"),
            Self::En => write!(f, "en"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
