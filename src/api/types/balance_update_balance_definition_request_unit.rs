pub use crate::prelude::*;

/// Unit of balance measurement.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateBalanceDefinitionRequestUnit {
    Points,
    Eur,
    Usd,
    Mxn,
    Gbp,
    Inr,
    Cad,
    Sgd,
    Ron,
    Jpy,
    Myr,
    Clp,
    Pen,
    Mad,
    Aud,
    Chf,
    Brl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateBalanceDefinitionRequestUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Points => serializer.serialize_str("POINTS"),
            Self::Eur => serializer.serialize_str("EUR"),
            Self::Usd => serializer.serialize_str("USD"),
            Self::Mxn => serializer.serialize_str("MXN"),
            Self::Gbp => serializer.serialize_str("GBP"),
            Self::Inr => serializer.serialize_str("INR"),
            Self::Cad => serializer.serialize_str("CAD"),
            Self::Sgd => serializer.serialize_str("SGD"),
            Self::Ron => serializer.serialize_str("RON"),
            Self::Jpy => serializer.serialize_str("JPY"),
            Self::Myr => serializer.serialize_str("MYR"),
            Self::Clp => serializer.serialize_str("CLP"),
            Self::Pen => serializer.serialize_str("PEN"),
            Self::Mad => serializer.serialize_str("MAD"),
            Self::Aud => serializer.serialize_str("AUD"),
            Self::Chf => serializer.serialize_str("CHF"),
            Self::Brl => serializer.serialize_str("BRL"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateBalanceDefinitionRequestUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "POINTS" => Ok(Self::Points),
            "EUR" => Ok(Self::Eur),
            "USD" => Ok(Self::Usd),
            "MXN" => Ok(Self::Mxn),
            "GBP" => Ok(Self::Gbp),
            "INR" => Ok(Self::Inr),
            "CAD" => Ok(Self::Cad),
            "SGD" => Ok(Self::Sgd),
            "RON" => Ok(Self::Ron),
            "JPY" => Ok(Self::Jpy),
            "MYR" => Ok(Self::Myr),
            "CLP" => Ok(Self::Clp),
            "PEN" => Ok(Self::Pen),
            "MAD" => Ok(Self::Mad),
            "AUD" => Ok(Self::Aud),
            "CHF" => Ok(Self::Chf),
            "BRL" => Ok(Self::Brl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateBalanceDefinitionRequestUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Points => write!(f, "POINTS"),
            Self::Eur => write!(f, "EUR"),
            Self::Usd => write!(f, "USD"),
            Self::Mxn => write!(f, "MXN"),
            Self::Gbp => write!(f, "GBP"),
            Self::Inr => write!(f, "INR"),
            Self::Cad => write!(f, "CAD"),
            Self::Sgd => write!(f, "SGD"),
            Self::Ron => write!(f, "RON"),
            Self::Jpy => write!(f, "JPY"),
            Self::Myr => write!(f, "MYR"),
            Self::Clp => write!(f, "CLP"),
            Self::Pen => write!(f, "PEN"),
            Self::Mad => write!(f, "MAD"),
            Self::Aud => write!(f, "AUD"),
            Self::Chf => write!(f, "CHF"),
            Self::Brl => write!(f, "BRL"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
