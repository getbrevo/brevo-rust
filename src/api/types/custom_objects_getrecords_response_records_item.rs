pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetrecordsResponseRecordsItem {
    /// List of associations for the object record. If association query param is true it will return 5 associated records per association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<GetrecordsResponseRecordsItemAssociationsItem>>,
    /// Key-value pairs of attribute data for the object record. Each key is the attribute **key** (e.g., `company_name`, `engine_type`), not the attribute label (e.g., "Company Name", "Engine Type"). Only attributes that already exist in the object schema will be present.
    /// For `category` or `multiple_category` attributes, the value is the option **key** (not the option label or option ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Timestamp when the object record was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Identifiers for the object record. Supports `id` (singular, not `ids`) or `ext_id`.
    /// - `ext_id`: Your external system's identifier for this record.
    /// - `id`: Internal Brevo record ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<GetrecordsResponseRecordsItemIdentifiers>,
    /// Timestamp when the object record was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl GetrecordsResponseRecordsItem {
    pub fn builder() -> GetrecordsResponseRecordsItemBuilder {
        <GetrecordsResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemBuilder {
    associations: Option<Vec<GetrecordsResponseRecordsItemAssociationsItem>>,
    attributes: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<DateTime<FixedOffset>>,
    identifiers: Option<GetrecordsResponseRecordsItemIdentifiers>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GetrecordsResponseRecordsItemBuilder {
    pub fn associations(
        mut self,
        value: Vec<GetrecordsResponseRecordsItemAssociationsItem>,
    ) -> Self {
        self.associations = Some(value);
        self
    }

    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn identifiers(mut self, value: GetrecordsResponseRecordsItemIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItem`].
    pub fn build(self) -> Result<GetrecordsResponseRecordsItem, BuildError> {
        Ok(GetrecordsResponseRecordsItem {
            associations: self.associations,
            attributes: self.attributes,
            created_at: self.created_at,
            identifiers: self.identifiers,
            updated_at: self.updated_at,
        })
    }
}
