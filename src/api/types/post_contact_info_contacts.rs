pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostContactInfoContacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<PostContactInfoContactsFailure>,
    /// Id of the process created to remove contacts from list when user opts for "all" option.
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<PostContactInfoContactsSuccess>,
    /// Displays the count of total number of contacts removed from list when user opts for "all" option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl PostContactInfoContacts {
    pub fn builder() -> PostContactInfoContactsBuilder {
        <PostContactInfoContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostContactInfoContactsBuilder {
    failure: Option<PostContactInfoContactsFailure>,
    process_id: Option<i64>,
    success: Option<PostContactInfoContactsSuccess>,
    total: Option<i64>,
}

impl PostContactInfoContactsBuilder {
    pub fn failure(mut self, value: PostContactInfoContactsFailure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    pub fn success(mut self, value: PostContactInfoContactsSuccess) -> Self {
        self.success = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostContactInfoContacts`].
    pub fn build(self) -> Result<PostContactInfoContacts, BuildError> {
        Ok(PostContactInfoContacts {
            failure: self.failure,
            process_id: self.process_id,
            success: self.success,
            total: self.total,
        })
    }
}
