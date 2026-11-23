pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContacts {
    #[serde(default)]
    pub contacts: Vec<GetContactDetails>,
    /// Number of contacts
    #[serde(default)]
    pub count: i64,
}

impl GetContacts {
    pub fn builder() -> GetContactsBuilder {
        <GetContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactsBuilder {
    contacts: Option<Vec<GetContactDetails>>,
    count: Option<i64>,
}

impl GetContactsBuilder {
    pub fn contacts(mut self, value: Vec<GetContactDetails>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContacts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts`](GetContactsBuilder::contacts)
    /// - [`count`](GetContactsBuilder::count)
    pub fn build(self) -> Result<GetContacts, BuildError> {
        Ok(GetContacts {
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
