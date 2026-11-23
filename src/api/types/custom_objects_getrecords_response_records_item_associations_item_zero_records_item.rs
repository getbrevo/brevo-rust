pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem {
    GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero(
        GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero,
    ),

    GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne(
        GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne,
    ),
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem {
    pub fn is_getrecords_response_records_item_associations_item_zero_records_item_zero(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero(_)
        )
    }

    pub fn is_getrecords_response_records_item_associations_item_zero_records_item_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne(_)
        )
    }

    pub fn as_getrecords_response_records_item_associations_item_zero_records_item_zero(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_zero_records_item_zero(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_getrecords_response_records_item_associations_item_zero_records_item_one(
        &self,
    ) -> Option<&GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_getrecords_response_records_item_associations_item_zero_records_item_one(
        self,
    ) -> Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne> {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
