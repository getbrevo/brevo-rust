pub use crate::prelude::*;

/// Query parameters for getEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEventsQueryRequest {
    /// Filter by contact ID (repeatable)
    #[serde(default)]
    pub contact_id: Vec<Option<i64>>,
    /// Filter by event name (repeatable)
    #[serde(default)]
    pub event_name: Vec<Option<String>>,
    /// Filter by object type (repeatable)
    #[serde(default)]
    pub object_type: Vec<Option<String>>,
    /// Mandatory if endDate is used. Start of date range (YYYY-MM-DD or RFC3339). Defaults to 6 months ago when omitted alongside endDate. Must be ≤ endDate.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Mandatory if startDate is used. End of date range (YYYY-MM-DD or RFC3339). Must be ≥ startDate.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Max events to return. Default 100, min 1, max 10000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Events to skip for pagination. Default 0, min 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetEventsQueryRequest {
    pub fn builder() -> GetEventsQueryRequestBuilder {
        <GetEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEventsQueryRequestBuilder {
    contact_id: Option<Vec<Option<i64>>>,
    event_name: Option<Vec<Option<String>>>,
    object_type: Option<Vec<Option<String>>>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl GetEventsQueryRequestBuilder {
    pub fn contact_id(mut self, value: Vec<Option<i64>>) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn event_name(mut self, value: Vec<Option<String>>) -> Self {
        self.event_name = Some(value);
        self
    }

    pub fn object_type(mut self, value: Vec<Option<String>>) -> Self {
        self.object_type = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEventsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_id`](GetEventsQueryRequestBuilder::contact_id)
    /// - [`event_name`](GetEventsQueryRequestBuilder::event_name)
    /// - [`object_type`](GetEventsQueryRequestBuilder::object_type)
    pub fn build(self) -> Result<GetEventsQueryRequest, BuildError> {
        Ok(GetEventsQueryRequest {
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            object_type: self
                .object_type
                .ok_or_else(|| BuildError::missing_field("object_type"))?,
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
