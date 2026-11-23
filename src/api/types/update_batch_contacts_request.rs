pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBatchContactsRequest {
    /// List of contacts to be updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<UpdateBatchContactsRequestContactsItem>>,
}

impl UpdateBatchContactsRequest {
    pub fn builder() -> UpdateBatchContactsRequestBuilder {
        <UpdateBatchContactsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBatchContactsRequestBuilder {
    contacts: Option<Vec<UpdateBatchContactsRequestContactsItem>>,
}

impl UpdateBatchContactsRequestBuilder {
    pub fn contacts(mut self, value: Vec<UpdateBatchContactsRequestContactsItem>) -> Self {
        self.contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBatchContactsRequest`].
    pub fn build(self) -> Result<UpdateBatchContactsRequest, BuildError> {
        Ok(UpdateBatchContactsRequest {
            contacts: self.contacts,
        })
    }
}
