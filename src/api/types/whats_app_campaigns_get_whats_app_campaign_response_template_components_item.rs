pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppCampaignResponseTemplateComponentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetWhatsAppCampaignResponseTemplateComponentsItem {
    pub fn builder() -> GetWhatsAppCampaignResponseTemplateComponentsItemBuilder {
        <GetWhatsAppCampaignResponseTemplateComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignResponseTemplateComponentsItemBuilder {
    text: Option<String>,
    r#type: Option<String>,
}

impl GetWhatsAppCampaignResponseTemplateComponentsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignResponseTemplateComponentsItem`].
    pub fn build(self) -> Result<GetWhatsAppCampaignResponseTemplateComponentsItem, BuildError> {
        Ok(GetWhatsAppCampaignResponseTemplateComponentsItem {
            text: self.text,
            r#type: self.r#type,
        })
    }
}
