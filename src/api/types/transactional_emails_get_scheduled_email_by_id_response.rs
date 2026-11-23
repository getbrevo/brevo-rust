pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GetScheduledEmailByIdResponse {
    GetScheduledEmailByIdResponseBatches(GetScheduledEmailByIdResponseBatches),

    GetScheduledEmailByIdResponseCreatedAt(GetScheduledEmailByIdResponseCreatedAt),
}

impl GetScheduledEmailByIdResponse {
    pub fn is_get_scheduled_email_by_id_response_batches(&self) -> bool {
        matches!(self, Self::GetScheduledEmailByIdResponseBatches(_))
    }

    pub fn is_get_scheduled_email_by_id_response_created_at(&self) -> bool {
        matches!(self, Self::GetScheduledEmailByIdResponseCreatedAt(_))
    }

    pub fn as_get_scheduled_email_by_id_response_batches(
        &self,
    ) -> Option<&GetScheduledEmailByIdResponseBatches> {
        match self {
            Self::GetScheduledEmailByIdResponseBatches(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_scheduled_email_by_id_response_batches(
        self,
    ) -> Option<GetScheduledEmailByIdResponseBatches> {
        match self {
            Self::GetScheduledEmailByIdResponseBatches(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_get_scheduled_email_by_id_response_created_at(
        &self,
    ) -> Option<&GetScheduledEmailByIdResponseCreatedAt> {
        match self {
            Self::GetScheduledEmailByIdResponseCreatedAt(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_scheduled_email_by_id_response_created_at(
        self,
    ) -> Option<GetScheduledEmailByIdResponseCreatedAt> {
        match self {
            Self::GetScheduledEmailByIdResponseCreatedAt(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetScheduledEmailByIdResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetScheduledEmailByIdResponseBatches(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GetScheduledEmailByIdResponseCreatedAt(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
