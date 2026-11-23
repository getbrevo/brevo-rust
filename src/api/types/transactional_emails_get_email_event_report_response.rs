pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailEventReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GetEmailEventReportResponseEventsItem>>,
}

impl GetEmailEventReportResponse {
    pub fn builder() -> GetEmailEventReportResponseBuilder {
        <GetEmailEventReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailEventReportResponseBuilder {
    events: Option<Vec<GetEmailEventReportResponseEventsItem>>,
}

impl GetEmailEventReportResponseBuilder {
    pub fn events(mut self, value: Vec<GetEmailEventReportResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailEventReportResponse`].
    pub fn build(self) -> Result<GetEmailEventReportResponse, BuildError> {
        Ok(GetEmailEventReportResponse {
            events: self.events,
        })
    }
}
