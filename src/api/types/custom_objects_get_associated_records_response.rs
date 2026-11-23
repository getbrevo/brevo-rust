pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAssociatedRecordsResponse {
    /// `true` when at least one more associated record exists after this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// Associated records for this page, up to 20 per page. Fewer when the last page is reached, empty when the source record has no matching associations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GetAssociatedRecordsResponseItemsItem>>,
    /// The offset that was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetAssociatedRecordsResponse {
    pub fn builder() -> GetAssociatedRecordsResponseBuilder {
        <GetAssociatedRecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssociatedRecordsResponseBuilder {
    has_more: Option<bool>,
    items: Option<Vec<GetAssociatedRecordsResponseItemsItem>>,
    offset: Option<i64>,
}

impl GetAssociatedRecordsResponseBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<GetAssociatedRecordsResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAssociatedRecordsResponse`].
    pub fn build(self) -> Result<GetAssociatedRecordsResponse, BuildError> {
        Ok(GetAssociatedRecordsResponse {
            has_more: self.has_more,
            items: self.items,
            offset: self.offset,
        })
    }
}
