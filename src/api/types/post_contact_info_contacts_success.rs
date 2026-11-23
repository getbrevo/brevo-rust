pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PostContactInfoContactsSuccess {
    StringList(Vec<String>),

    LongList(Vec<i64>),
}

impl PostContactInfoContactsSuccess {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_long_list(&self) -> bool {
        matches!(self, Self::LongList(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_long_list(&self) -> Option<&Vec<i64>> {
        match self {
            Self::LongList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_long_list(self) -> Option<Vec<i64>> {
        match self {
            Self::LongList(value) => Some(value),
            _ => None,
        }
    }
}
