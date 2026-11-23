pub use crate::prelude::*;

/// Custom attributes for the report email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendReportEmail {
    /// Custom text message to be presented in the report email.
    #[serde(default)]
    pub body: String,
    /// Email addresses of the recipients
    #[serde(default)]
    pub to: Vec<String>,
}

impl SendReportEmail {
    pub fn builder() -> SendReportEmailBuilder {
        <SendReportEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendReportEmailBuilder {
    body: Option<String>,
    to: Option<Vec<String>>,
}

impl SendReportEmailBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn to(mut self, value: Vec<String>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendReportEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](SendReportEmailBuilder::body)
    /// - [`to`](SendReportEmailBuilder::to)
    pub fn build(self) -> Result<SendReportEmail, BuildError> {
        Ok(SendReportEmail {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
