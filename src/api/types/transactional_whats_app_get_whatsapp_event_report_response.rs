pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsappEventReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GetWhatsappEventReportResponseEventsItem>>,
}

impl GetWhatsappEventReportResponse {
    pub fn builder() -> GetWhatsappEventReportResponseBuilder {
        <GetWhatsappEventReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsappEventReportResponseBuilder {
    events: Option<Vec<GetWhatsappEventReportResponseEventsItem>>,
}

impl GetWhatsappEventReportResponseBuilder {
    pub fn events(mut self, value: Vec<GetWhatsappEventReportResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsappEventReportResponse`].
    pub fn build(self) -> Result<GetWhatsappEventReportResponse, BuildError> {
        Ok(GetWhatsappEventReportResponse {
            events: self.events,
        })
    }
}
