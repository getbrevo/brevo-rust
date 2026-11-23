pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacEmailsListResponse {
    /// Total number of transactional emails available on your account according to the passed filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "transactionalEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_emails: Option<Vec<GetTransacEmailsListResponseTransactionalEmailsItem>>,
}

impl GetTransacEmailsListResponse {
    pub fn builder() -> GetTransacEmailsListResponseBuilder {
        <GetTransacEmailsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacEmailsListResponseBuilder {
    count: Option<i64>,
    transactional_emails: Option<Vec<GetTransacEmailsListResponseTransactionalEmailsItem>>,
}

impl GetTransacEmailsListResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn transactional_emails(
        mut self,
        value: Vec<GetTransacEmailsListResponseTransactionalEmailsItem>,
    ) -> Self {
        self.transactional_emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacEmailsListResponse`].
    pub fn build(self) -> Result<GetTransacEmailsListResponse, BuildError> {
        Ok(GetTransacEmailsListResponse {
            count: self.count,
            transactional_emails: self.transactional_emails,
        })
    }
}
