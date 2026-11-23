pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacSmsReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<GetTransacSmsReportResponseReportsItem>>,
}

impl GetTransacSmsReportResponse {
    pub fn builder() -> GetTransacSmsReportResponseBuilder {
        <GetTransacSmsReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacSmsReportResponseBuilder {
    reports: Option<Vec<GetTransacSmsReportResponseReportsItem>>,
}

impl GetTransacSmsReportResponseBuilder {
    pub fn reports(mut self, value: Vec<GetTransacSmsReportResponseReportsItem>) -> Self {
        self.reports = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacSmsReportResponse`].
    pub fn build(self) -> Result<GetTransacSmsReportResponse, BuildError> {
        Ok(GetTransacSmsReportResponse {
            reports: self.reports,
        })
    }
}
