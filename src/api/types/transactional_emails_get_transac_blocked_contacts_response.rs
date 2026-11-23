pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacBlockedContactsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<GetTransacBlockedContactsResponseContactsItem>>,
    /// Count of blocked or unsubscribed contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetTransacBlockedContactsResponse {
    pub fn builder() -> GetTransacBlockedContactsResponseBuilder {
        <GetTransacBlockedContactsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacBlockedContactsResponseBuilder {
    contacts: Option<Vec<GetTransacBlockedContactsResponseContactsItem>>,
    count: Option<i64>,
}

impl GetTransacBlockedContactsResponseBuilder {
    pub fn contacts(mut self, value: Vec<GetTransacBlockedContactsResponseContactsItem>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacBlockedContactsResponse`].
    pub fn build(self) -> Result<GetTransacBlockedContactsResponse, BuildError> {
        Ok(GetTransacBlockedContactsResponse {
            contacts: self.contacts,
            count: self.count,
        })
    }
}
