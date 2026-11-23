pub use crate::prelude::*;

/// Query parameters for deleteAnSmtpTransactionalLog
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAnSmtpTransactionalLogQueryRequest {
    /// Starting date (YYYY-MM-DD) to narrow down logs for deletion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// Ending date (YYYY-MM-DD) to narrow down logs for deletion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
}

impl DeleteAnSmtpTransactionalLogQueryRequest {
    pub fn builder() -> DeleteAnSmtpTransactionalLogQueryRequestBuilder {
        <DeleteAnSmtpTransactionalLogQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAnSmtpTransactionalLogQueryRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
}

impl DeleteAnSmtpTransactionalLogQueryRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAnSmtpTransactionalLogQueryRequest`].
    pub fn build(self) -> Result<DeleteAnSmtpTransactionalLogQueryRequest, BuildError> {
        Ok(DeleteAnSmtpTransactionalLogQueryRequest {
            from_date: self.from_date,
            to_date: self.to_date,
        })
    }
}
