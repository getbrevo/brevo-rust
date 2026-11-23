pub use crate::prelude::*;

/// A contact's membership status in a single consent group.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsentGroupItem {
    /// ID of the consent group
    #[serde(default)]
    pub id: i64,
    /// The contact's subscription status within this consent group
    pub status: ConsentGroupItemStatus,
}

impl ConsentGroupItem {
    pub fn builder() -> ConsentGroupItemBuilder {
        <ConsentGroupItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsentGroupItemBuilder {
    id: Option<i64>,
    status: Option<ConsentGroupItemStatus>,
}

impl ConsentGroupItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(mut self, value: ConsentGroupItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsentGroupItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConsentGroupItemBuilder::id)
    /// - [`status`](ConsentGroupItemBuilder::status)
    pub fn build(self) -> Result<ConsentGroupItem, BuildError> {
        Ok(ConsentGroupItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
