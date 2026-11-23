pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateSmtpTemplateRequestTemplateId {
    Long(i64),

    String(String),
}

impl UpdateSmtpTemplateRequestTemplateId {
    pub fn is_long(&self) -> bool {
        matches!(self, Self::Long(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_long(&self) -> Option<&i64> {
        match self {
            Self::Long(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_long(self) -> Option<i64> {
        match self {
            Self::Long(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdateSmtpTemplateRequestTemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Long(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
