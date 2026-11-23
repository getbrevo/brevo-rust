pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountActivityResponseLogsItem {
    /// Type of activity in the account.
    #[serde(default)]
    pub action: String,
    /// Time of the activity.
    #[serde(default)]
    pub date: String,
    /// Browser details of the user who performed the activity.
    #[serde(default)]
    pub user_agent: String,
    /// Email address of the user who performed activity in the
    /// account.
    #[serde(default)]
    pub user_email: String,
    /// IP address of the user who performed activity in the
    /// account.
    #[serde(default)]
    pub user_ip: String,
}

impl GetAccountActivityResponseLogsItem {
    pub fn builder() -> GetAccountActivityResponseLogsItemBuilder {
        <GetAccountActivityResponseLogsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountActivityResponseLogsItemBuilder {
    action: Option<String>,
    date: Option<String>,
    user_agent: Option<String>,
    user_email: Option<String>,
    user_ip: Option<String>,
}

impl GetAccountActivityResponseLogsItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn user_email(mut self, value: impl Into<String>) -> Self {
        self.user_email = Some(value.into());
        self
    }

    pub fn user_ip(mut self, value: impl Into<String>) -> Self {
        self.user_ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountActivityResponseLogsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](GetAccountActivityResponseLogsItemBuilder::action)
    /// - [`date`](GetAccountActivityResponseLogsItemBuilder::date)
    /// - [`user_agent`](GetAccountActivityResponseLogsItemBuilder::user_agent)
    /// - [`user_email`](GetAccountActivityResponseLogsItemBuilder::user_email)
    /// - [`user_ip`](GetAccountActivityResponseLogsItemBuilder::user_ip)
    pub fn build(self) -> Result<GetAccountActivityResponseLogsItem, BuildError> {
        Ok(GetAccountActivityResponseLogsItem {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            user_agent: self
                .user_agent
                .ok_or_else(|| BuildError::missing_field("user_agent"))?,
            user_email: self
                .user_email
                .ok_or_else(|| BuildError::missing_field("user_email"))?,
            user_ip: self
                .user_ip
                .ok_or_else(|| BuildError::missing_field("user_ip"))?,
        })
    }
}
