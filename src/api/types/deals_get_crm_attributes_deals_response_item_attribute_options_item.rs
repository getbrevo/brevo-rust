pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmAttributesDealsResponseItemAttributeOptionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetCrmAttributesDealsResponseItemAttributeOptionsItem {
    pub fn builder() -> GetCrmAttributesDealsResponseItemAttributeOptionsItemBuilder {
        <GetCrmAttributesDealsResponseItemAttributeOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmAttributesDealsResponseItemAttributeOptionsItemBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl GetCrmAttributesDealsResponseItemAttributeOptionsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCrmAttributesDealsResponseItemAttributeOptionsItem`].
    pub fn build(
        self,
    ) -> Result<GetCrmAttributesDealsResponseItemAttributeOptionsItem, BuildError> {
        Ok(GetCrmAttributesDealsResponseItemAttributeOptionsItem {
            key: self.key,
            value: self.value,
        })
    }
}
