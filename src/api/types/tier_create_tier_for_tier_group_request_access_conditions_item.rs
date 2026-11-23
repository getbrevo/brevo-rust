pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTierForTierGroupRequestAccessConditionsItem {
    /// Balance definition identifier for accessing the tier
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Minimum value required to access the tier
    #[serde(rename = "minimumValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<i64>,
}

impl CreateTierForTierGroupRequestAccessConditionsItem {
    pub fn builder() -> CreateTierForTierGroupRequestAccessConditionsItemBuilder {
        <CreateTierForTierGroupRequestAccessConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierForTierGroupRequestAccessConditionsItemBuilder {
    balance_definition_id: Option<String>,
    minimum_value: Option<i64>,
}

impl CreateTierForTierGroupRequestAccessConditionsItemBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn minimum_value(mut self, value: i64) -> Self {
        self.minimum_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTierForTierGroupRequestAccessConditionsItem`].
    pub fn build(self) -> Result<CreateTierForTierGroupRequestAccessConditionsItem, BuildError> {
        Ok(CreateTierForTierGroupRequestAccessConditionsItem {
            balance_definition_id: self.balance_definition_id,
            minimum_value: self.minimum_value,
        })
    }
}
