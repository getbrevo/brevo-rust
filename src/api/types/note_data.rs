pub use crate::prelude::*;

/// Note data to be saved
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NoteData {
    /// Company Ids linked to a note
    #[serde(rename = "companyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ids: Option<Vec<String>>,
    /// Contact Ids linked to a note
    #[serde(rename = "contactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_ids: Option<Vec<i64>>,
    /// Deal Ids linked to a note
    #[serde(rename = "dealIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_ids: Option<Vec<String>>,
    /// Content of the note. Supports HTML for rich text formatting. Supported tags include: `<p>` (paragraph), `<b>` / `<strong>` (bold), `<i>` / `<em>` (italic), `<u>` (underline), `<br>` (line break), `<a href="...">` (labelled hyperlink). Example labelled link: `<a href="https://example.com">Link text</a>`.
    #[serde(default)]
    pub text: String,
}

impl NoteData {
    pub fn builder() -> NoteDataBuilder {
        <NoteDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NoteDataBuilder {
    company_ids: Option<Vec<String>>,
    contact_ids: Option<Vec<i64>>,
    deal_ids: Option<Vec<String>>,
    text: Option<String>,
}

impl NoteDataBuilder {
    pub fn company_ids(mut self, value: Vec<String>) -> Self {
        self.company_ids = Some(value);
        self
    }

    pub fn contact_ids(mut self, value: Vec<i64>) -> Self {
        self.contact_ids = Some(value);
        self
    }

    pub fn deal_ids(mut self, value: Vec<String>) -> Self {
        self.deal_ids = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NoteData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](NoteDataBuilder::text)
    pub fn build(self) -> Result<NoteData, BuildError> {
        Ok(NoteData {
            company_ids: self.company_ids,
            contact_ids: self.contact_ids,
            deal_ids: self.deal_ids,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
