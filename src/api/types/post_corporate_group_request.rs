pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateGroupRequest {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName")]
    #[serde(default)]
    pub group_name: String,
    /// Pass the list of sub-account Ids to be included in the group
    #[serde(rename = "subAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_ids: Option<Vec<i64>>,
}

impl PostCorporateGroupRequest {
    pub fn builder() -> PostCorporateGroupRequestBuilder {
        <PostCorporateGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateGroupRequestBuilder {
    group_name: Option<String>,
    sub_account_ids: Option<Vec<i64>>,
}

impl PostCorporateGroupRequestBuilder {
    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn sub_account_ids(mut self, value: Vec<i64>) -> Self {
        self.sub_account_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_name`](PostCorporateGroupRequestBuilder::group_name)
    pub fn build(self) -> Result<PostCorporateGroupRequest, BuildError> {
        Ok(PostCorporateGroupRequest {
            group_name: self
                .group_name
                .ok_or_else(|| BuildError::missing_field("group_name"))?,
            sub_account_ids: self.sub_account_ids,
        })
    }
}
