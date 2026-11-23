pub use crate::prelude::*;

/// Sales and service users to use phone, sales and
/// conversations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers {
    /// Quantity of sales users provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available sales users for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsersBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsersBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsersBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}
