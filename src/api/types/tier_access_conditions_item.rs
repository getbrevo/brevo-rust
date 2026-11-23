pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TierAccessConditionsItem {
    /// Balance definition identifier
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Minimum value required to access this tier
    #[serde(rename = "minimumValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<i64>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl TierAccessConditionsItem {
    pub fn builder() -> TierAccessConditionsItemBuilder {
        <TierAccessConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierAccessConditionsItemBuilder {
    balance_definition_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    minimum_value: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TierAccessConditionsItemBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn minimum_value(mut self, value: i64) -> Self {
        self.minimum_value = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TierAccessConditionsItem`].
    pub fn build(self) -> Result<TierAccessConditionsItem, BuildError> {
        Ok(TierAccessConditionsItem {
            balance_definition_id: self.balance_definition_id,
            created_at: self.created_at,
            minimum_value: self.minimum_value,
            updated_at: self.updated_at,
        })
    }
}
