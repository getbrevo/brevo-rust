pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSenderResponse {
    /// Status of DKIM configuration for the sender (true = DKIM not
    /// well configured, false = DKIM well configured)
    #[serde(rename = "dkimError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_error: Option<bool>,
    /// ID of the Sender created
    #[serde(default)]
    pub id: i64,
    /// Status of SPF configuration for the sender (true = SPF not
    /// well configured, false = SPF well configured)
    #[serde(rename = "spfError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_error: Option<bool>,
}

impl CreateSenderResponse {
    pub fn builder() -> CreateSenderResponseBuilder {
        <CreateSenderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSenderResponseBuilder {
    dkim_error: Option<bool>,
    id: Option<i64>,
    spf_error: Option<bool>,
}

impl CreateSenderResponseBuilder {
    pub fn dkim_error(mut self, value: bool) -> Self {
        self.dkim_error = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn spf_error(mut self, value: bool) -> Self {
        self.spf_error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSenderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSenderResponseBuilder::id)
    pub fn build(self) -> Result<CreateSenderResponse, BuildError> {
        Ok(CreateSenderResponse {
            dkim_error: self.dkim_error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            spf_error: self.spf_error,
        })
    }
}
