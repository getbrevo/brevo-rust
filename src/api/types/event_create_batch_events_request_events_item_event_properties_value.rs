pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateBatchEventsRequestEventsItemEventPropertiesValue {
    String(String),

    Integer(i64),

    StringToValueMap(HashMap<String, serde_json::Value>),

    ValueList(Vec<serde_json::Value>),
}

impl CreateBatchEventsRequestEventsItemEventPropertiesValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_value_list(&self) -> bool {
        matches!(self, Self::ValueList(_))
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

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_value_list(&self) -> Option<&Vec<serde_json::Value>> {
        match self {
            Self::ValueList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_value_list(self) -> Option<Vec<serde_json::Value>> {
        match self {
            Self::ValueList(value) => Some(value),
            _ => None,
        }
    }
}
