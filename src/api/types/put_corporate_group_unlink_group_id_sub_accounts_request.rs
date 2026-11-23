pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
    /// List of sub-account ids
    #[serde(rename = "subAccountIds")]
    #[serde(default)]
    pub sub_account_ids: Vec<i64>,
}

impl PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
    pub fn builder() -> PutCorporateGroupUnlinkGroupIdSubAccountsRequestBuilder {
        <PutCorporateGroupUnlinkGroupIdSubAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateGroupUnlinkGroupIdSubAccountsRequestBuilder {
    sub_account_ids: Option<Vec<i64>>,
}

impl PutCorporateGroupUnlinkGroupIdSubAccountsRequestBuilder {
    pub fn sub_account_ids(mut self, value: Vec<i64>) -> Self {
        self.sub_account_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateGroupUnlinkGroupIdSubAccountsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sub_account_ids`](PutCorporateGroupUnlinkGroupIdSubAccountsRequestBuilder::sub_account_ids)
    pub fn build(self) -> Result<PutCorporateGroupUnlinkGroupIdSubAccountsRequest, BuildError> {
        Ok(PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
            sub_account_ids: self
                .sub_account_ids
                .ok_or_else(|| BuildError::missing_field("sub_account_ids"))?,
        })
    }
}
