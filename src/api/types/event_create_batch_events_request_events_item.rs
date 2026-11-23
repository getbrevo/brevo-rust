pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBatchEventsRequestEventsItem {
    /// Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **"FIRSTNAME": "Jane" , "AGE": 37**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_properties:
        Option<HashMap<String, CreateBatchEventsRequestEventsItemContactPropertiesValue>>,
    /// Timestamp of when the event occurred (e.g. "2024-01-24T17:39:57+01:00"). If no value is passed, the timestamp of the event creation is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    /// The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters, alphanumerical characters and - _ only.
    #[serde(default)]
    pub event_name: String,
    /// Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field type are supported: string, number, boolean (true/false), date (Timestamp e.g. "2024-01-24T17:39:57+01:00"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50Kb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_properties:
        Option<HashMap<String, CreateBatchEventsRequestEventsItemEventPropertiesValue>>,
    /// Identifies the contact associated with the event. At least one identifier is required.
    #[serde(default)]
    pub identifiers: CreateBatchEventsRequestEventsItemIdentifiers,
    /// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CreateBatchEventsRequestEventsItemObject>,
}

impl CreateBatchEventsRequestEventsItem {
    pub fn builder() -> CreateBatchEventsRequestEventsItemBuilder {
        <CreateBatchEventsRequestEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchEventsRequestEventsItemBuilder {
    contact_properties:
        Option<HashMap<String, CreateBatchEventsRequestEventsItemContactPropertiesValue>>,
    event_date: Option<String>,
    event_name: Option<String>,
    event_properties:
        Option<HashMap<String, CreateBatchEventsRequestEventsItemEventPropertiesValue>>,
    identifiers: Option<CreateBatchEventsRequestEventsItemIdentifiers>,
    object: Option<CreateBatchEventsRequestEventsItemObject>,
}

impl CreateBatchEventsRequestEventsItemBuilder {
    pub fn contact_properties(
        mut self,
        value: HashMap<String, CreateBatchEventsRequestEventsItemContactPropertiesValue>,
    ) -> Self {
        self.contact_properties = Some(value);
        self
    }

    pub fn event_date(mut self, value: impl Into<String>) -> Self {
        self.event_date = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn event_properties(
        mut self,
        value: HashMap<String, CreateBatchEventsRequestEventsItemEventPropertiesValue>,
    ) -> Self {
        self.event_properties = Some(value);
        self
    }

    pub fn identifiers(mut self, value: CreateBatchEventsRequestEventsItemIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn object(mut self, value: CreateBatchEventsRequestEventsItemObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchEventsRequestEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](CreateBatchEventsRequestEventsItemBuilder::event_name)
    /// - [`identifiers`](CreateBatchEventsRequestEventsItemBuilder::identifiers)
    pub fn build(self) -> Result<CreateBatchEventsRequestEventsItem, BuildError> {
        Ok(CreateBatchEventsRequestEventsItem {
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
