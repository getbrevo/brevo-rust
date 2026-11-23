pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GetrecordsResponseRecordsItemAssociationsItem {
    GetrecordsResponseRecordsItemAssociationsItemZero(
        GetrecordsResponseRecordsItemAssociationsItemZero,
    ),

    GetrecordsResponseRecordsItemAssociationsItemOne(
        GetrecordsResponseRecordsItemAssociationsItemOne,
    ),
}

impl GetrecordsResponseRecordsItemAssociationsItem {
    pub fn is_getrecords_response_records_item_associations_item_zero(&self) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemZero(_)
        )
    }

    pub fn is_getrecords_response_records_item_associations_item_one(&self) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemOne(_)
        )
    }

    pub fn as_getrecords_response_records_item_associations_item_zero(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_zero(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_getrecords_response_records_item_associations_item_one(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_one(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetrecordsResponseRecordsItemAssociationsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GetrecordsResponseRecordsItemAssociationsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
