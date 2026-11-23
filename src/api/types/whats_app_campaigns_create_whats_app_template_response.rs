pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWhatsAppTemplateResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateWhatsAppTemplateResponse {
    pub fn builder() -> CreateWhatsAppTemplateResponseBuilder {
        <CreateWhatsAppTemplateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWhatsAppTemplateResponseBuilder {
    id: Option<i64>,
}

impl CreateWhatsAppTemplateResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWhatsAppTemplateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWhatsAppTemplateResponseBuilder::id)
    pub fn build(self) -> Result<CreateWhatsAppTemplateResponse, BuildError> {
        Ok(CreateWhatsAppTemplateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
