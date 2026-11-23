pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetList {
    /// ID of the list
    #[serde(default)]
    pub id: i64,
    /// Name of the list
    #[serde(default)]
    pub name: String,
    /// Number of blacklisted contacts in the list
    #[serde(rename = "totalBlacklisted")]
    #[serde(default)]
    pub total_blacklisted: i64,
    /// Number of contacts in the list
    #[serde(rename = "totalSubscribers")]
    #[serde(default)]
    pub total_subscribers: i64,
    /// Number of unique contacts in the list
    #[serde(rename = "uniqueSubscribers")]
    #[serde(default)]
    pub unique_subscribers: i64,
}

impl GetList {
    pub fn builder() -> GetListBuilder {
        <GetListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListBuilder {
    id: Option<i64>,
    name: Option<String>,
    total_blacklisted: Option<i64>,
    total_subscribers: Option<i64>,
    unique_subscribers: Option<i64>,
}

impl GetListBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn total_blacklisted(mut self, value: i64) -> Self {
        self.total_blacklisted = Some(value);
        self
    }

    pub fn total_subscribers(mut self, value: i64) -> Self {
        self.total_subscribers = Some(value);
        self
    }

    pub fn unique_subscribers(mut self, value: i64) -> Self {
        self.unique_subscribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetListBuilder::id)
    /// - [`name`](GetListBuilder::name)
    /// - [`total_blacklisted`](GetListBuilder::total_blacklisted)
    /// - [`total_subscribers`](GetListBuilder::total_subscribers)
    /// - [`unique_subscribers`](GetListBuilder::unique_subscribers)
    pub fn build(self) -> Result<GetList, BuildError> {
        Ok(GetList {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            total_blacklisted: self
                .total_blacklisted
                .ok_or_else(|| BuildError::missing_field("total_blacklisted"))?,
            total_subscribers: self
                .total_subscribers
                .ok_or_else(|| BuildError::missing_field("total_subscribers"))?,
            unique_subscribers: self
                .unique_subscribers
                .ok_or_else(|| BuildError::missing_field("unique_subscribers"))?,
        })
    }
}
