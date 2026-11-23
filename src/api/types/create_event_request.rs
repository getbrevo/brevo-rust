pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateEventRequest {
    /// Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **"FIRSTNAME": "Jane" , "AGE": 37**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_properties: Option<HashMap<String, CreateEventRequestContactPropertiesValue>>,
    /// ISO 8601 timestamp of when the event occurred (e.g. "2024-01-24T17:39:57+01:00"). If no value is passed, the timestamp of the event creation is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_date: Option<DateTime<FixedOffset>>,
    /// The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters; only alphanumeric characters, hyphens (-), and underscores (_) are allowed.
    #[serde(default)]
    pub event_name: String,
    /// Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field types are supported: string, number, boolean (true/false), date (Timestamp e.g. "2024-01-24T17:39:57+01:00"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50KB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_properties: Option<HashMap<String, CreateEventRequestEventPropertiesValue>>,
    /// Identifies the contact associated with the event. At least one identifier is required.
    #[serde(default)]
    pub identifiers: CreateEventRequestIdentifiers,
    /// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CreateEventRequestObject>,
}

impl CreateEventRequest {
    pub fn builder() -> CreateEventRequestBuilder {
        <CreateEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventRequestBuilder {
    contact_properties: Option<HashMap<String, CreateEventRequestContactPropertiesValue>>,
    event_date: Option<DateTime<FixedOffset>>,
    event_name: Option<String>,
    event_properties: Option<HashMap<String, CreateEventRequestEventPropertiesValue>>,
    identifiers: Option<CreateEventRequestIdentifiers>,
    object: Option<CreateEventRequestObject>,
}

impl CreateEventRequestBuilder {
    pub fn contact_properties(
        mut self,
        value: HashMap<String, CreateEventRequestContactPropertiesValue>,
    ) -> Self {
        self.contact_properties = Some(value);
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

    pub fn event_properties(
        mut self,
        value: HashMap<String, CreateEventRequestEventPropertiesValue>,
    ) -> Self {
        self.event_properties = Some(value);
        self
    }

    pub fn identifiers(mut self, value: CreateEventRequestIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn object(mut self, value: CreateEventRequestObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEventRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](CreateEventRequestBuilder::event_name)
    /// - [`identifiers`](CreateEventRequestBuilder::identifiers)
    pub fn build(self) -> Result<CreateEventRequest, BuildError> {
        Ok(CreateEventRequest {
            contact_properties: self.contact_properties,
            event_date: self.event_date,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            event_properties: self.event_properties,
            identifiers: self
                .identifiers
                .ok_or_else(|| BuildError::missing_field("identifiers"))?,
            object: self.object,
        })
    }
}
