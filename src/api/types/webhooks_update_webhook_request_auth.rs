pub use crate::prelude::*;

/// Add authentication on webhook url
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookRequestAuth {
    /// Webhook authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Type of authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl UpdateWebhookRequestAuth {
    pub fn builder() -> UpdateWebhookRequestAuthBuilder {
        <UpdateWebhookRequestAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookRequestAuthBuilder {
    token: Option<String>,
    r#type: Option<String>,
}

impl UpdateWebhookRequestAuthBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookRequestAuth`].
    pub fn build(self) -> Result<UpdateWebhookRequestAuth, BuildError> {
        Ok(UpdateWebhookRequestAuth {
            token: self.token,
            r#type: self.r#type,
        })
    }
}
