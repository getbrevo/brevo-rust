pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacEmailContentResponseEventsItem {
    /// Name of the event that occurred on the sent email
    #[serde(default)]
    pub name: String,
    /// Time at which the event occurred
    #[serde(default)]
    pub time: String,
}

impl GetTransacEmailContentResponseEventsItem {
    pub fn builder() -> GetTransacEmailContentResponseEventsItemBuilder {
        <GetTransacEmailContentResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacEmailContentResponseEventsItemBuilder {
    name: Option<String>,
    time: Option<String>,
}

impl GetTransacEmailContentResponseEventsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn time(mut self, value: impl Into<String>) -> Self {
        self.time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTransacEmailContentResponseEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetTransacEmailContentResponseEventsItemBuilder::name)
    /// - [`time`](GetTransacEmailContentResponseEventsItemBuilder::time)
    pub fn build(self) -> Result<GetTransacEmailContentResponseEventsItem, BuildError> {
        Ok(GetTransacEmailContentResponseEventsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            time: self.time.ok_or_else(|| BuildError::missing_field("time"))?,
        })
    }
}
