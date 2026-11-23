pub use crate::prelude::*;

/// Explicit transaction type. If not provided, the type is inferred from the sign of the amount (positive = credit, negative = debit).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BeginTransactionRequestTransactionType {
    Credit,
    Debit,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BeginTransactionRequestTransactionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Credit => serializer.serialize_str("credit"),
            Self::Debit => serializer.serialize_str("debit"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BeginTransactionRequestTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credit" => Ok(Self::Credit),
            "debit" => Ok(Self::Debit),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BeginTransactionRequestTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credit => write!(f, "credit"),
            Self::Debit => write!(f, "debit"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
