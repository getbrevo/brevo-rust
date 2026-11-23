pub use crate::prelude::*;

/// Modifier for balance availability duration. startOfPeriod causes expiry at the start of the period, endOfPeriod at the end. noModification uses the exact duration from the credit date.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BalanceDefinitionBalanceAvailabilityDurationModifier {
    NoModification,
    StartOfPeriod,
    EndOfPeriod,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BalanceDefinitionBalanceAvailabilityDurationModifier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoModification => serializer.serialize_str("noModification"),
            Self::StartOfPeriod => serializer.serialize_str("startOfPeriod"),
            Self::EndOfPeriod => serializer.serialize_str("endOfPeriod"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BalanceDefinitionBalanceAvailabilityDurationModifier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "noModification" => Ok(Self::NoModification),
            "startOfPeriod" => Ok(Self::StartOfPeriod),
            "endOfPeriod" => Ok(Self::EndOfPeriod),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BalanceDefinitionBalanceAvailabilityDurationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoModification => write!(f, "noModification"),
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
