pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateGroupIdRequest {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Pass the list of sub-account Ids to be included in the group
    #[serde(rename = "subAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_ids: Option<Vec<i64>>,
}

impl PutCorporateGroupIdRequest {
    pub fn builder() -> PutCorporateGroupIdRequestBuilder {
        <PutCorporateGroupIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateGroupIdRequestBuilder {
    group_name: Option<String>,
    sub_account_ids: Option<Vec<i64>>,
}

impl PutCorporateGroupIdRequestBuilder {
    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn sub_account_ids(mut self, value: Vec<i64>) -> Self {
        self.sub_account_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateGroupIdRequest`].
    pub fn build(self) -> Result<PutCorporateGroupIdRequest, BuildError> {
        Ok(PutCorporateGroupIdRequest {
            group_name: self.group_name,
            sub_account_ids: self.sub_account_ids,
        })
    }
}
