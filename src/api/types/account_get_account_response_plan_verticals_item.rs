pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAccountResponsePlanVerticalsItem {
    /// Category of the plan
    #[serde(rename = "planCategory")]
    pub plan_category: GetAccountResponsePlanVerticalsItemPlanCategory,
    /// Type of plan within the category
    #[serde(rename = "planType")]
    #[serde(default)]
    pub plan_type: String,
    /// Name of the plan
    #[serde(default)]
    pub name: String,
    /// Current status of the plan
    pub status: GetAccountResponsePlanVerticalsItemStatus,
    /// Plan start date (Unix timestamp)
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Plan end date (Unix timestamp)
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// User seat information for the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<GetAccountResponsePlanVerticalsItemUsers>,
    /// Credits allocated for this plan category (optional, may not be present for all plan categories)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<String>,
}

impl GetAccountResponsePlanVerticalsItem {
    pub fn builder() -> GetAccountResponsePlanVerticalsItemBuilder {
        <GetAccountResponsePlanVerticalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponsePlanVerticalsItemBuilder {
    plan_category: Option<GetAccountResponsePlanVerticalsItemPlanCategory>,
    plan_type: Option<String>,
    name: Option<String>,
    status: Option<GetAccountResponsePlanVerticalsItemStatus>,
    start_date: Option<String>,
    end_date: Option<String>,
    users: Option<GetAccountResponsePlanVerticalsItemUsers>,
    credits: Option<String>,
}

impl GetAccountResponsePlanVerticalsItemBuilder {
    pub fn plan_category(mut self, value: GetAccountResponsePlanVerticalsItemPlanCategory) -> Self {
        self.plan_category = Some(value);
        self
    }

    pub fn plan_type(mut self, value: impl Into<String>) -> Self {
        self.plan_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetAccountResponsePlanVerticalsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn users(mut self, value: GetAccountResponsePlanVerticalsItemUsers) -> Self {
        self.users = Some(value);
        self
    }

    pub fn credits(mut self, value: impl Into<String>) -> Self {
        self.credits = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponsePlanVerticalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan_category`](GetAccountResponsePlanVerticalsItemBuilder::plan_category)
    /// - [`plan_type`](GetAccountResponsePlanVerticalsItemBuilder::plan_type)
    /// - [`name`](GetAccountResponsePlanVerticalsItemBuilder::name)
    /// - [`status`](GetAccountResponsePlanVerticalsItemBuilder::status)
    pub fn build(self) -> Result<GetAccountResponsePlanVerticalsItem, BuildError> {
        Ok(GetAccountResponsePlanVerticalsItem {
            plan_category: self
                .plan_category
                .ok_or_else(|| BuildError::missing_field("plan_category"))?,
            plan_type: self
                .plan_type
                .ok_or_else(|| BuildError::missing_field("plan_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            start_date: self.start_date,
            end_date: self.end_date,
            users: self.users,
            credits: self.credits,
        })
    }
}
