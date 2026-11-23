pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpTemplateOverviewSender {
    /// From email for the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Sender id of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Sender name for the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetSmtpTemplateOverviewSender {
    pub fn builder() -> GetSmtpTemplateOverviewSenderBuilder {
        <GetSmtpTemplateOverviewSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpTemplateOverviewSenderBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl GetSmtpTemplateOverviewSenderBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpTemplateOverviewSender`].
    pub fn build(self) -> Result<GetSmtpTemplateOverviewSender, BuildError> {
        Ok(GetSmtpTemplateOverviewSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
