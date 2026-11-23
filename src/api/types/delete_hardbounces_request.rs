pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteHardbouncesRequest {
    /// Target a specific email address
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Ending date (YYYY-MM-DD) of the time period for deletion. The hardbounces until this date will be deleted. Must be greater than or equal to the startDate
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Starting date (YYYY-MM-DD) of the time period for deletion. The hardbounces occurred after this date will be deleted. Must be less than or equal to the endDate
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl DeleteHardbouncesRequest {
    pub fn builder() -> DeleteHardbouncesRequestBuilder {
        <DeleteHardbouncesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteHardbouncesRequestBuilder {
    contact_email: Option<String>,
    end_date: Option<String>,
    start_date: Option<String>,
}

impl DeleteHardbouncesRequestBuilder {
    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteHardbouncesRequest`].
    pub fn build(self) -> Result<DeleteHardbouncesRequest, BuildError> {
        Ok(DeleteHardbouncesRequest {
            contact_email: self.contact_email,
            end_date: self.end_date,
            start_date: self.start_date,
        })
    }
}
