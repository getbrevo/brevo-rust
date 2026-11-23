pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BatchDeleteObjectRecordsRequestIdentifiers {
    BatchDeleteObjectRecordsRequestIdentifiersIds(BatchDeleteObjectRecordsRequestIdentifiersIds),

    BatchDeleteObjectRecordsRequestIdentifiersExtIds(
        BatchDeleteObjectRecordsRequestIdentifiersExtIds,
    ),
}

impl BatchDeleteObjectRecordsRequestIdentifiers {
    pub fn is_batch_delete_object_records_request_identifiers_ids(&self) -> bool {
        matches!(self, Self::BatchDeleteObjectRecordsRequestIdentifiersIds(_))
    }

    pub fn is_batch_delete_object_records_request_identifiers_ext_ids(&self) -> bool {
        matches!(
            self,
            Self::BatchDeleteObjectRecordsRequestIdentifiersExtIds(_)
        )
    }

    pub fn as_batch_delete_object_records_request_identifiers_ids(
        &self,
    ) -> Option<&BatchDeleteObjectRecordsRequestIdentifiersIds> {
        match self {
            Self::BatchDeleteObjectRecordsRequestIdentifiersIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_batch_delete_object_records_request_identifiers_ids(
        self,
    ) -> Option<BatchDeleteObjectRecordsRequestIdentifiersIds> {
        match self {
            Self::BatchDeleteObjectRecordsRequestIdentifiersIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_batch_delete_object_records_request_identifiers_ext_ids(
        &self,
    ) -> Option<&BatchDeleteObjectRecordsRequestIdentifiersExtIds> {
        match self {
            Self::BatchDeleteObjectRecordsRequestIdentifiersExtIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_batch_delete_object_records_request_identifiers_ext_ids(
        self,
    ) -> Option<BatchDeleteObjectRecordsRequestIdentifiersExtIds> {
        match self {
            Self::BatchDeleteObjectRecordsRequestIdentifiersExtIds(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BatchDeleteObjectRecordsRequestIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BatchDeleteObjectRecordsRequestIdentifiersIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BatchDeleteObjectRecordsRequestIdentifiersExtIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
