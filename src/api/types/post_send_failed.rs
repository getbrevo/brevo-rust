pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostSendFailed {
    #[serde(rename = "blackListedEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_listed_emails: Option<Vec<String>>,
    /// Response code
    #[serde(default)]
    pub code: String,
    /// Response message
    #[serde(default)]
    pub message: String,
    #[serde(rename = "unexistingEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unexisting_emails: Option<Vec<String>>,
    #[serde(rename = "withoutListEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_list_emails: Option<Vec<String>>,
}

impl PostSendFailed {
    pub fn builder() -> PostSendFailedBuilder {
        <PostSendFailedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostSendFailedBuilder {
    black_listed_emails: Option<Vec<String>>,
    code: Option<String>,
    message: Option<String>,
    unexisting_emails: Option<Vec<String>>,
    without_list_emails: Option<Vec<String>>,
}

impl PostSendFailedBuilder {
    pub fn black_listed_emails(mut self, value: Vec<String>) -> Self {
        self.black_listed_emails = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn unexisting_emails(mut self, value: Vec<String>) -> Self {
        self.unexisting_emails = Some(value);
        self
    }

    pub fn without_list_emails(mut self, value: Vec<String>) -> Self {
        self.without_list_emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostSendFailed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostSendFailedBuilder::code)
    /// - [`message`](PostSendFailedBuilder::message)
    pub fn build(self) -> Result<PostSendFailed, BuildError> {
        Ok(PostSendFailed {
            black_listed_emails: self.black_listed_emails,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            unexisting_emails: self.unexisting_emails,
            without_list_emails: self.without_list_emails,
        })
    }
}
