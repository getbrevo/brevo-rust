pub use crate::prelude::*;

/// Marketing users to manage the marketing channels
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers {
    /// Quantity of marketing users provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available marketing users for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoFeaturesUsersBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoFeaturesUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesUsersBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesUsersBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}
