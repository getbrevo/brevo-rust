pub use crate::prelude::*;

/// Query parameters for getTheListOfAllTheSubAccountsOfTheMasterAccount
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest {
    /// Index of the first sub-account in the page
    #[serde(default)]
    pub offset: i64,
    /// Number of sub-accounts to be displayed on each page
    #[serde(default)]
    pub limit: i64,
}

impl GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest {
    pub fn builder() -> GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder {
        <GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder {
    offset: Option<i64>,
    limit: Option<i64>,
}

impl GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder {
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`offset`](GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder::offset)
    /// - [`limit`](GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequestBuilder::limit)
    pub fn build(
        self,
    ) -> Result<GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest, BuildError> {
        Ok(
            GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest {
                offset: self
                    .offset
                    .ok_or_else(|| BuildError::missing_field("offset"))?,
                limit: self
                    .limit
                    .ok_or_else(|| BuildError::missing_field("limit"))?,
            },
        )
    }
}
