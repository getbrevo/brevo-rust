pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppCampaignResponseTemplate {
    /// array of variables item variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_variables: Option<Vec<VariablesItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_type: Option<String>,
    /// description of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// array of component item objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<GetWhatsAppCampaignResponseTemplateComponentsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_button: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_header: Option<bool>,
    /// type of header
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_type: Option<String>,
    /// array of variables item object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_variables: Option<Vec<VariablesItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_footer: Option<bool>,
    /// language of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// name of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetWhatsAppCampaignResponseTemplate {
    pub fn builder() -> GetWhatsAppCampaignResponseTemplateBuilder {
        <GetWhatsAppCampaignResponseTemplateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignResponseTemplateBuilder {
    body_variables: Option<Vec<VariablesItems>>,
    button_type: Option<String>,
    category: Option<String>,
    components: Option<Vec<GetWhatsAppCampaignResponseTemplateComponentsItem>>,
    contains_button: Option<bool>,
    display_header: Option<bool>,
    header_type: Option<String>,
    header_variables: Option<Vec<VariablesItems>>,
    hide_footer: Option<bool>,
    language: Option<String>,
    name: Option<String>,
}

impl GetWhatsAppCampaignResponseTemplateBuilder {
    pub fn body_variables(mut self, value: Vec<VariablesItems>) -> Self {
        self.body_variables = Some(value);
        self
    }

    pub fn button_type(mut self, value: impl Into<String>) -> Self {
        self.button_type = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn components(
        mut self,
        value: Vec<GetWhatsAppCampaignResponseTemplateComponentsItem>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn contains_button(mut self, value: bool) -> Self {
        self.contains_button = Some(value);
        self
    }

    pub fn display_header(mut self, value: bool) -> Self {
        self.display_header = Some(value);
        self
    }

    pub fn header_type(mut self, value: impl Into<String>) -> Self {
        self.header_type = Some(value.into());
        self
    }

    pub fn header_variables(mut self, value: Vec<VariablesItems>) -> Self {
        self.header_variables = Some(value);
        self
    }

    pub fn hide_footer(mut self, value: bool) -> Self {
        self.hide_footer = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignResponseTemplate`].
    pub fn build(self) -> Result<GetWhatsAppCampaignResponseTemplate, BuildError> {
        Ok(GetWhatsAppCampaignResponseTemplate {
            body_variables: self.body_variables,
            button_type: self.button_type,
            category: self.category,
            components: self.components,
            contains_button: self.contains_button,
            display_header: self.display_header,
            header_type: self.header_type,
            header_variables: self.header_variables,
            hide_footer: self.hide_footer,
            language: self.language,
            name: self.name,
        })
    }
}
