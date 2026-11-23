pub use crate::prelude::*;

/// User seat information for the plan
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponsePlanVerticalsItemUsers {
    /// Number of purchased user seats
    #[serde(rename = "purchasedSeats")]
    #[serde(default)]
    pub purchased_seats: String,
    /// Number of used user seats
    #[serde(rename = "usedSeats")]
    #[serde(default)]
    pub used_seats: String,
}

impl GetAccountResponsePlanVerticalsItemUsers {
    pub fn builder() -> GetAccountResponsePlanVerticalsItemUsersBuilder {
        <GetAccountResponsePlanVerticalsItemUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponsePlanVerticalsItemUsersBuilder {
    purchased_seats: Option<String>,
    used_seats: Option<String>,
}

impl GetAccountResponsePlanVerticalsItemUsersBuilder {
    pub fn purchased_seats(mut self, value: impl Into<String>) -> Self {
        self.purchased_seats = Some(value.into());
        self
    }

    pub fn used_seats(mut self, value: impl Into<String>) -> Self {
        self.used_seats = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponsePlanVerticalsItemUsers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`purchased_seats`](GetAccountResponsePlanVerticalsItemUsersBuilder::purchased_seats)
    /// - [`used_seats`](GetAccountResponsePlanVerticalsItemUsersBuilder::used_seats)
    pub fn build(self) -> Result<GetAccountResponsePlanVerticalsItemUsers, BuildError> {
        Ok(GetAccountResponsePlanVerticalsItemUsers {
            purchased_seats: self
                .purchased_seats
                .ok_or_else(|| BuildError::missing_field("purchased_seats"))?,
            used_seats: self
                .used_seats
                .ok_or_else(|| BuildError::missing_field("used_seats"))?,
        })
    }
}
