pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetScheduledEmailByIdResponseCreatedAt {
    /// Datetime on which the email was scheduled
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Datetime for which the email was scheduled
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_at: DateTime<FixedOffset>,
    /// Current status of the scheduled email
    pub status: GetScheduledEmailByIdResponseCreatedAtStatus,
}

impl GetScheduledEmailByIdResponseCreatedAt {
    pub fn builder() -> GetScheduledEmailByIdResponseCreatedAtBuilder {
        <GetScheduledEmailByIdResponseCreatedAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetScheduledEmailByIdResponseCreatedAtBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    scheduled_at: Option<DateTime<FixedOffset>>,
    status: Option<GetScheduledEmailByIdResponseCreatedAtStatus>,
}

impl GetScheduledEmailByIdResponseCreatedAtBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn scheduled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    pub fn status(mut self, value: GetScheduledEmailByIdResponseCreatedAtStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetScheduledEmailByIdResponseCreatedAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetScheduledEmailByIdResponseCreatedAtBuilder::created_at)
    /// - [`scheduled_at`](GetScheduledEmailByIdResponseCreatedAtBuilder::scheduled_at)
    /// - [`status`](GetScheduledEmailByIdResponseCreatedAtBuilder::status)
    pub fn build(self) -> Result<GetScheduledEmailByIdResponseCreatedAt, BuildError> {
        Ok(GetScheduledEmailByIdResponseCreatedAt {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            scheduled_at: self
                .scheduled_at
                .ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
