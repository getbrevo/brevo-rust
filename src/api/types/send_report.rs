pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendReport {
    /// Custom attributes for the report email.
    #[serde(default)]
    pub email: SendReportEmail,
    /// Language of email content for campaign report sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<SendReportLanguage>,
}

impl SendReport {
    pub fn builder() -> SendReportBuilder {
        <SendReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendReportBuilder {
    email: Option<SendReportEmail>,
    language: Option<SendReportLanguage>,
}

impl SendReportBuilder {
    pub fn email(mut self, value: SendReportEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn language(mut self, value: SendReportLanguage) -> Self {
        self.language = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendReport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](SendReportBuilder::email)
    pub fn build(self) -> Result<SendReport, BuildError> {
        Ok(SendReport {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            language: self.language,
        })
    }
}
