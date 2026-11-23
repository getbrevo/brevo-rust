pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEventsListEventsItem {
    /// Contact ID associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Date and time of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_date: Option<DateTime<FixedOffset>>,
    /// Name of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// Filter ID of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter_id: Option<String>,
    /// Object type associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// Event-level custom properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_properties: Option<HashMap<String, serde_json::Value>>,
    /// Contact-level properties at time of event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_properties: Option<HashMap<String, serde_json::Value>>,
}

impl GetEventsListEventsItem {
    pub fn builder() -> GetEventsListEventsItemBuilder {
        <GetEventsListEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEventsListEventsItemBuilder {
    contact_id: Option<i64>,
    event_date: Option<DateTime<FixedOffset>>,
    event_name: Option<String>,
    event_filter_id: Option<String>,
    object_type: Option<String>,
    event_properties: Option<HashMap<String, serde_json::Value>>,
    contact_properties: Option<HashMap<String, serde_json::Value>>,
}

impl GetEventsListEventsItemBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn event_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_date = Some(value);
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn event_filter_id(mut self, value: impl Into<String>) -> Self {
        self.event_filter_id = Some(value.into());
        self
    }

    pub fn object_type(mut self, value: impl Into<String>) -> Self {
        self.object_type = Some(value.into());
        self
    }

    pub fn event_properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.event_properties = Some(value);
        self
    }

    pub fn contact_properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.contact_properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEventsListEventsItem`].
    pub fn build(self) -> Result<GetEventsListEventsItem, BuildError> {
        Ok(GetEventsListEventsItem {
            contact_id: self.contact_id,
            event_date: self.event_date,
            event_name: self.event_name,
            event_filter_id: self.event_filter_id,
            object_type: self.object_type,
            event_properties: self.event_properties,
            contact_properties: self.contact_properties,
        })
    }
}
