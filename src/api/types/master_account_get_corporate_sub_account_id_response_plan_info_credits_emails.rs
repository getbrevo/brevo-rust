pub use crate::prelude::*;

/// Email credits remaining on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsEmails {
    /// Quantity of email messaging limits provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available email messaging limits for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsEmails {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsEmailsBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsEmailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsEmailsBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsEmailsBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCreditsEmails`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoCreditsEmails, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoCreditsEmails {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}
