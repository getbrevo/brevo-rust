pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RemoveContactFromListRequestBodyEmails {
    /// **Required if 'all' is false and 'ids', 'extIds' are empty.** Emails to remove from a list. You can pass a **maximum of 150 emails** for removal in one request.
    pub emails: Option<Vec<String>>,
}

impl RemoveContactFromListRequestBodyEmails {
    pub fn builder() -> RemoveContactFromListRequestBodyEmailsBuilder {
        <RemoveContactFromListRequestBodyEmailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveContactFromListRequestBodyEmailsBuilder {
    emails: Option<Vec<String>>,
}

impl RemoveContactFromListRequestBodyEmailsBuilder {
    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveContactFromListRequestBodyEmails`].
    pub fn build(self) -> Result<RemoveContactFromListRequestBodyEmails, BuildError> {
        Ok(RemoveContactFromListRequestBodyEmails {
            emails: self.emails,
        })
    }
}
