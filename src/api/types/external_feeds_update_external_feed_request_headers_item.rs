pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExternalFeedRequestHeadersItem {
    /// Header name
    #[serde(default)]
    pub name: String,
    /// Header value
    #[serde(default)]
    pub value: String,
}

impl UpdateExternalFeedRequestHeadersItem {
    pub fn builder() -> UpdateExternalFeedRequestHeadersItemBuilder {
        <UpdateExternalFeedRequestHeadersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExternalFeedRequestHeadersItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl UpdateExternalFeedRequestHeadersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateExternalFeedRequestHeadersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateExternalFeedRequestHeadersItemBuilder::name)
    /// - [`value`](UpdateExternalFeedRequestHeadersItemBuilder::value)
    pub fn build(self) -> Result<UpdateExternalFeedRequestHeadersItem, BuildError> {
        Ok(UpdateExternalFeedRequestHeadersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
