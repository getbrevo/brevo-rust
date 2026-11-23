pub use crate::prelude::*;

/// Whatsapp credits remaining on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp {
    /// Quantity of whatsapp messaging limits provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available whatsapp messaging limits for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsappBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsappBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsappBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsappBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}
