pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSmtpTemplateResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateSmtpTemplateResponse {
    pub fn builder() -> CreateSmtpTemplateResponseBuilder {
        <CreateSmtpTemplateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSmtpTemplateResponseBuilder {
    id: Option<i64>,
}

impl CreateSmtpTemplateResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSmtpTemplateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSmtpTemplateResponseBuilder::id)
    pub fn build(self) -> Result<CreateSmtpTemplateResponse, BuildError> {
        Ok(CreateSmtpTemplateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
