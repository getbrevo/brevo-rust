pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AddContactToListRequestBodyEmails {
    /// Emails to add to a list. You can pass a **maximum of 150 emails** for addition in one request. **_If you need to add the emails in bulk, please prefer /contacts/import api._**
    pub emails: Option<Vec<String>>,
}

impl AddContactToListRequestBodyEmails {
    pub fn builder() -> AddContactToListRequestBodyEmailsBuilder {
        <AddContactToListRequestBodyEmailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddContactToListRequestBodyEmailsBuilder {
    emails: Option<Vec<String>>,
}

impl AddContactToListRequestBodyEmailsBuilder {
    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddContactToListRequestBodyEmails`].
    pub fn build(self) -> Result<AddContactToListRequestBodyEmails, BuildError> {
        Ok(AddContactToListRequestBodyEmails {
            emails: self.emails,
        })
    }
}
