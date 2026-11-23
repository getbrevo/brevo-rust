pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmAttributesCompaniesResponseItemAttributeOptionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetCrmAttributesCompaniesResponseItemAttributeOptionsItem {
    pub fn builder() -> GetCrmAttributesCompaniesResponseItemAttributeOptionsItemBuilder {
        <GetCrmAttributesCompaniesResponseItemAttributeOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmAttributesCompaniesResponseItemAttributeOptionsItemBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl GetCrmAttributesCompaniesResponseItemAttributeOptionsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCrmAttributesCompaniesResponseItemAttributeOptionsItem`].
    pub fn build(
        self,
    ) -> Result<GetCrmAttributesCompaniesResponseItemAttributeOptionsItem, BuildError> {
        Ok(GetCrmAttributesCompaniesResponseItemAttributeOptionsItem {
            key: self.key,
            value: self.value,
        })
    }
}
