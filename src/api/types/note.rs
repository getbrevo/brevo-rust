pub use crate::prelude::*;

/// Note Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Note {
    /// Unique note Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Content of the note. Supports HTML for rich text formatting. Supported tags include: `<p>` (paragraph), `<b>` / `<strong>` (bold), `<i>` / `<em>` (italic), `<u>` (underline), `<br>` (line break), `<a href="...">` (labelled hyperlink). Example labelled link: `<a href="https://example.com">Link text</a>`.
    #[serde(default)]
    pub text: String,
    /// Contact ids linked to a note
    #[serde(rename = "contactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_ids: Option<Vec<i64>>,
    /// Company ids linked to a note
    #[serde(rename = "companyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ids: Option<Vec<String>>,
    /// Deal ids linked to a note
    #[serde(rename = "dealIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_ids: Option<Vec<String>>,
    /// Account details of user which created the note
    #[serde(rename = "authorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<HashMap<String, serde_json::Value>>,
    /// Note created date/time
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Note updated date/time
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl Note {
    pub fn builder() -> NoteBuilder {
        <NoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NoteBuilder {
    id: Option<String>,
    text: Option<String>,
    contact_ids: Option<Vec<i64>>,
    company_ids: Option<Vec<String>>,
    deal_ids: Option<Vec<String>>,
    author_id: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl NoteBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn contact_ids(mut self, value: Vec<i64>) -> Self {
        self.contact_ids = Some(value);
        self
    }

    pub fn company_ids(mut self, value: Vec<String>) -> Self {
        self.company_ids = Some(value);
        self
    }

    pub fn deal_ids(mut self, value: Vec<String>) -> Self {
        self.deal_ids = Some(value);
        self
    }

    pub fn author_id(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.author_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Note`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](NoteBuilder::text)
    pub fn build(self) -> Result<Note, BuildError> {
        Ok(Note {
            id: self.id,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            contact_ids: self.contact_ids,
            company_ids: self.company_ids,
            deal_ids: self.deal_ids,
            author_id: self.author_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
