pub use crate::prelude::*;

/// Error code displayed in case of a failure
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContactErrorModelCode {
    InvalidParameter,
    MissingParameter,
    DocumentNotFound,
    AccountInProcess,
    DuplicateParameter,
    MethodNotAllowed,
    OutOfRange,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ContactErrorModelCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InvalidParameter => serializer.serialize_str("invalid_parameter"),
            Self::MissingParameter => serializer.serialize_str("missing_parameter"),
            Self::DocumentNotFound => serializer.serialize_str("document_not_found"),
            Self::AccountInProcess => serializer.serialize_str("account_in_process"),
            Self::DuplicateParameter => serializer.serialize_str("duplicate_parameter"),
            Self::MethodNotAllowed => serializer.serialize_str("method_not_allowed"),
            Self::OutOfRange => serializer.serialize_str("out_of_range"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContactErrorModelCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid_parameter" => Ok(Self::InvalidParameter),
            "missing_parameter" => Ok(Self::MissingParameter),
            "document_not_found" => Ok(Self::DocumentNotFound),
            "account_in_process" => Ok(Self::AccountInProcess),
            "duplicate_parameter" => Ok(Self::DuplicateParameter),
            "method_not_allowed" => Ok(Self::MethodNotAllowed),
            "out_of_range" => Ok(Self::OutOfRange),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ContactErrorModelCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter => write!(f, "invalid_parameter"),
            Self::MissingParameter => write!(f, "missing_parameter"),
            Self::DocumentNotFound => write!(f, "document_not_found"),
            Self::AccountInProcess => write!(f, "account_in_process"),
            Self::DuplicateParameter => write!(f, "duplicate_parameter"),
            Self::MethodNotAllowed => write!(f, "method_not_allowed"),
            Self::OutOfRange => write!(f, "out_of_range"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
