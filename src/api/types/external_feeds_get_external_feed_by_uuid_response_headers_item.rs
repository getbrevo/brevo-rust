pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetExternalFeedByUuidResponseHeadersItem {
    /// Header name
    #[serde(default)]
    pub name: String,
    /// Header value
    #[serde(default)]
    pub value: String,
}

impl GetExternalFeedByUuidResponseHeadersItem {
    pub fn builder() -> GetExternalFeedByUuidResponseHeadersItemBuilder {
        <GetExternalFeedByUuidResponseHeadersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExternalFeedByUuidResponseHeadersItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl GetExternalFeedByUuidResponseHeadersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetExternalFeedByUuidResponseHeadersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetExternalFeedByUuidResponseHeadersItemBuilder::name)
    /// - [`value`](GetExternalFeedByUuidResponseHeadersItemBuilder::value)
    pub fn build(self) -> Result<GetExternalFeedByUuidResponseHeadersItem, BuildError> {
        Ok(GetExternalFeedByUuidResponseHeadersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
