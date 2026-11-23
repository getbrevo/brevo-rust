pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAccountResponsePlanItem {
    /// Remaining credits of the user
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub credits: f64,
    /// This is the type of the credit, "Send Limit" is one
    /// of the possible types of credit of a user. "Send
    /// Limit" implies the total number of emails you can
    /// send to the subscribers in your account.
    #[serde(rename = "creditsType")]
    pub credits_type: String,
    /// Date of the period from which the plan will end
    /// (Unix timestamp format)
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Date of the period from which the plan will start
    /// (Unix timestamp format)
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Displays the plan type of the user
    pub r#type: GetAccountResponsePlanItemType,
}

impl GetAccountResponsePlanItem {
    pub fn builder() -> GetAccountResponsePlanItemBuilder {
        <GetAccountResponsePlanItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponsePlanItemBuilder {
    credits: Option<f64>,
    credits_type: Option<String>,
    end_date: Option<String>,
    start_date: Option<String>,
    r#type: Option<GetAccountResponsePlanItemType>,
}

impl GetAccountResponsePlanItemBuilder {
    pub fn credits(mut self, value: f64) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn credits_type(mut self, value: impl Into<String>) -> Self {
        self.credits_type = Some(value.into());
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

    pub fn r#type(mut self, value: GetAccountResponsePlanItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponsePlanItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credits`](GetAccountResponsePlanItemBuilder::credits)
    /// - [`credits_type`](GetAccountResponsePlanItemBuilder::credits_type)
    /// - [`r#type`](GetAccountResponsePlanItemBuilder::r#type)
    pub fn build(self) -> Result<GetAccountResponsePlanItem, BuildError> {
        Ok(GetAccountResponsePlanItem {
            credits: self
                .credits
                .ok_or_else(|| BuildError::missing_field("credits"))?,
            credits_type: self
                .credits_type
                .ok_or_else(|| BuildError::missing_field("credits_type"))?,
            end_date: self.end_date,
            start_date: self.start_date,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
