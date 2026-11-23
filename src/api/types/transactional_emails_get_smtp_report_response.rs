pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<GetSmtpReportResponseReportsItem>>,
}

impl GetSmtpReportResponse {
    pub fn builder() -> GetSmtpReportResponseBuilder {
        <GetSmtpReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpReportResponseBuilder {
    reports: Option<Vec<GetSmtpReportResponseReportsItem>>,
}

impl GetSmtpReportResponseBuilder {
    pub fn reports(mut self, value: Vec<GetSmtpReportResponseReportsItem>) -> Self {
        self.reports = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpReportResponse`].
    pub fn build(self) -> Result<GetSmtpReportResponse, BuildError> {
        Ok(GetSmtpReportResponse {
            reports: self.reports,
        })
    }
}
