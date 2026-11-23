pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetListsResponse {
    /// Number of lists in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Listing of all the lists available in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<GetListsResponseListsItem>>,
}

impl GetListsResponse {
    pub fn builder() -> GetListsResponseBuilder {
        <GetListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListsResponseBuilder {
    count: Option<i64>,
    lists: Option<Vec<GetListsResponseListsItem>>,
}

impl GetListsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn lists(mut self, value: Vec<GetListsResponseListsItem>) -> Self {
        self.lists = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetListsResponse`].
    pub fn build(self) -> Result<GetListsResponse, BuildError> {
        Ok(GetListsResponse {
            count: self.count,
            lists: self.lists,
        })
    }
}
