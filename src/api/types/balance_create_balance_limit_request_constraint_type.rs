pub use crate::prelude::*;

/// Defines whether the limit applies to transaction count or amount.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBalanceLimitRequestConstraintType {
    Transaction,
    Amount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBalanceLimitRequestConstraintType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transaction => serializer.serialize_str("transaction"),
            Self::Amount => serializer.serialize_str("amount"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBalanceLimitRequestConstraintType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transaction" => Ok(Self::Transaction),
            "amount" => Ok(Self::Amount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBalanceLimitRequestConstraintType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction => write!(f, "transaction"),
            Self::Amount => write!(f, "amount"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
