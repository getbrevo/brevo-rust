pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem {
    GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero(
        GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero,
    ),

    GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne(
        GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne,
    ),
}

impl GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem {
    pub fn is_getrecords_response_records_item_associations_item_one_records_item_zero(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero(_)
        )
    }

    pub fn is_getrecords_response_records_item_associations_item_one_records_item_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne(_)
        )
    }

    pub fn as_getrecords_response_records_item_associations_item_one_records_item_zero(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_one_records_item_zero(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_getrecords_response_records_item_associations_item_one_records_item_one(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_one_records_item_one(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
