pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTestEmail {
    /// List of the email addresses of the recipients whom you wish to send the test mail. _If left empty, the test mail will be sent to your entire test list. You can not send more than 50 test emails per day_.
    #[serde(rename = "emailTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_to: Option<Vec<String>>,
}

impl SendTestEmail {
    pub fn builder() -> SendTestEmailBuilder {
        <SendTestEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTestEmailBuilder {
    email_to: Option<Vec<String>>,
}

impl SendTestEmailBuilder {
    pub fn email_to(mut self, value: Vec<String>) -> Self {
        self.email_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTestEmail`].
    pub fn build(self) -> Result<SendTestEmail, BuildError> {
        Ok(SendTestEmail {
            email_to: self.email_to,
        })
    }
}
