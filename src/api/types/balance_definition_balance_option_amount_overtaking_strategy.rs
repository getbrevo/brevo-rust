pub use crate::prelude::*;

/// Partial enables partial credit of balance if maximum balance limit is reached. Strict enables rejection of a transaction if it will breach the max balance limit.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    Strict,
    Partial,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Strict => serializer.serialize_str("strict"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "strict" => Ok(Self::Strict),
            "partial" => Ok(Self::Partial),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BalanceDefinitionBalanceOptionAmountOvertakingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Partial => write!(f, "partial"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
