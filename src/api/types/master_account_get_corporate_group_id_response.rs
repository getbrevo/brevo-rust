pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateGroupIdResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GetCorporateGroupIdResponseGroup>,
    #[serde(rename = "sub-accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_accounts: Option<Vec<GetCorporateGroupIdResponseSubAccountsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GetCorporateGroupIdResponseUsersItem>>,
}

impl GetCorporateGroupIdResponse {
    pub fn builder() -> GetCorporateGroupIdResponseBuilder {
        <GetCorporateGroupIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateGroupIdResponseBuilder {
    group: Option<GetCorporateGroupIdResponseGroup>,
    sub_accounts: Option<Vec<GetCorporateGroupIdResponseSubAccountsItem>>,
    users: Option<Vec<GetCorporateGroupIdResponseUsersItem>>,
}

impl GetCorporateGroupIdResponseBuilder {
    pub fn group(mut self, value: GetCorporateGroupIdResponseGroup) -> Self {
        self.group = Some(value);
        self
    }

    pub fn sub_accounts(mut self, value: Vec<GetCorporateGroupIdResponseSubAccountsItem>) -> Self {
        self.sub_accounts = Some(value);
        self
    }

    pub fn users(mut self, value: Vec<GetCorporateGroupIdResponseUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateGroupIdResponse`].
    pub fn build(self) -> Result<GetCorporateGroupIdResponse, BuildError> {
        Ok(GetCorporateGroupIdResponse {
            group: self.group,
            sub_accounts: self.sub_accounts,
            users: self.users,
        })
    }
}
