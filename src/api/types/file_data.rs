pub use crate::prelude::*;

/// File data that is uploaded
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileData {
    /// Unique file id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of uploaded file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Account id of user which created the file
    #[serde(rename = "authorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// Contact id of contact on which file is uploaded
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Deal id linked to a file
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<String>,
    /// Company id linked to a file
    #[serde(rename = "companyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Size of file in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// File created date/time
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
}

impl FileData {
    pub fn builder() -> FileDataBuilder {
        <FileDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileDataBuilder {
    id: Option<String>,
    name: Option<String>,
    author_id: Option<String>,
    contact_id: Option<i64>,
    deal_id: Option<String>,
    company_id: Option<String>,
    size: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl FileDataBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn author_id(mut self, value: impl Into<String>) -> Self {
        self.author_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn deal_id(mut self, value: impl Into<String>) -> Self {
        self.deal_id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileData`].
    pub fn build(self) -> Result<FileData, BuildError> {
        Ok(FileData {
            id: self.id,
            name: self.name,
            author_id: self.author_id,
            contact_id: self.contact_id,
            deal_id: self.deal_id,
            company_id: self.company_id,
            size: self.size,
            created_at: self.created_at,
        })
    }
}
