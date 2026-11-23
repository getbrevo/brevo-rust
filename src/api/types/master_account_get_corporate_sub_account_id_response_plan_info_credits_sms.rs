pub use crate::prelude::*;

/// SMS credits remaining on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsSms {
    /// Quantity of SMS messaging limits provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available SMS messaging limits for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsSms {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsSmsBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsSmsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsSmsBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsSmsBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCreditsSms`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponsePlanInfoCreditsSms, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoCreditsSms {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}
