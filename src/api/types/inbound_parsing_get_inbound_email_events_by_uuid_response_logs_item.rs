pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsByUuidResponseLogsItem {
    /// Date of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date: Option<DateTime<FixedOffset>>,
    /// Type of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GetInboundEmailEventsByUuidResponseLogsItemType>,
}

impl GetInboundEmailEventsByUuidResponseLogsItem {
    pub fn builder() -> GetInboundEmailEventsByUuidResponseLogsItemBuilder {
        <GetInboundEmailEventsByUuidResponseLogsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsByUuidResponseLogsItemBuilder {
    date: Option<DateTime<FixedOffset>>,
    r#type: Option<GetInboundEmailEventsByUuidResponseLogsItemType>,
}

impl GetInboundEmailEventsByUuidResponseLogsItemBuilder {
    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
        self
    }

    pub fn r#type(mut self, value: GetInboundEmailEventsByUuidResponseLogsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsByUuidResponseLogsItem`].
    pub fn build(self) -> Result<GetInboundEmailEventsByUuidResponseLogsItem, BuildError> {
        Ok(GetInboundEmailEventsByUuidResponseLogsItem {
            date: self.date,
            r#type: self.r#type,
        })
    }
}
