pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse {
    /// Unique identifier for the balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The current amount available in the balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Loyalty program ID.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Organization ID.
    #[serde(rename = "organizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<i64>,
    /// Contact ID.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Balance definition ID.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Timestamp of when the balance was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Expiration timestamp of the balance.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Timestamp of when the balance was last consumed.
    #[serde(rename = "consumedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub consumed_at: Option<DateTime<FixedOffset>>,
}

impl PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse {
    pub fn builder() -> PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponseBuilder {
        <PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponseBuilder {
    id: Option<String>,
    amount: Option<f64>,
    loyalty_program_id: Option<String>,
    organization_id: Option<i64>,
    contact_id: Option<i64>,
    balance_definition_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
    consumed_at: Option<DateTime<FixedOffset>>,
}

impl PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: i64) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn consumed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.consumed_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse`].
    pub fn build(
        self,
    ) -> Result<PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse, BuildError> {
        Ok(
            PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse {
                id: self.id,
                amount: self.amount,
                loyalty_program_id: self.loyalty_program_id,
                organization_id: self.organization_id,
                contact_id: self.contact_id,
                balance_definition_id: self.balance_definition_id,
                created_at: self.created_at,
                expires_at: self.expires_at,
                consumed_at: self.consumed_at,
            },
        )
    }
}
