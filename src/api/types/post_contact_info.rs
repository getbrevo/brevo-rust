pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostContactInfo {
    #[serde(default)]
    pub contacts: PostContactInfoContacts,
}

impl PostContactInfo {
    pub fn builder() -> PostContactInfoBuilder {
        <PostContactInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostContactInfoBuilder {
    contacts: Option<PostContactInfoContacts>,
}

impl PostContactInfoBuilder {
    pub fn contacts(mut self, value: PostContactInfoContacts) -> Self {
        self.contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostContactInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts`](PostContactInfoBuilder::contacts)
    pub fn build(self) -> Result<PostContactInfo, BuildError> {
        Ok(PostContactInfo {
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
        })
    }
}
