pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountResponse {
    /// Total number of subaccounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "subAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_accounts: Option<Vec<GetCorporateSubAccountResponseSubAccountsItem>>,
}

impl GetCorporateSubAccountResponse {
    pub fn builder() -> GetCorporateSubAccountResponseBuilder {
        <GetCorporateSubAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountResponseBuilder {
    count: Option<i64>,
    sub_accounts: Option<Vec<GetCorporateSubAccountResponseSubAccountsItem>>,
}

impl GetCorporateSubAccountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn sub_accounts(
        mut self,
        value: Vec<GetCorporateSubAccountResponseSubAccountsItem>,
    ) -> Self {
        self.sub_accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountResponse`].
    pub fn build(self) -> Result<GetCorporateSubAccountResponse, BuildError> {
        Ok(GetCorporateSubAccountResponse {
            count: self.count,
            sub_accounts: self.sub_accounts,
        })
    }
}
