pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PostPreviewSmtpEmailTemplatesRequest {
    Value(serde_json::Value),
}

impl PostPreviewSmtpEmailTemplatesRequest {
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_value(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
        }
    }

    pub fn into_value(self) -> Option<serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
        }
    }
}
