pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTierRequestAccessConditionsItem {
    /// Balance definition identifier for accessing the tier
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Minimum value required to access the tier
    #[serde(rename = "minimumValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<i64>,
}

impl UpdateTierRequestAccessConditionsItem {
    pub fn builder() -> UpdateTierRequestAccessConditionsItemBuilder {
        <UpdateTierRequestAccessConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTierRequestAccessConditionsItemBuilder {
    balance_definition_id: Option<String>,
    minimum_value: Option<i64>,
}

impl UpdateTierRequestAccessConditionsItemBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn minimum_value(mut self, value: i64) -> Self {
        self.minimum_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTierRequestAccessConditionsItem`].
    pub fn build(self) -> Result<UpdateTierRequestAccessConditionsItem, BuildError> {
        Ok(UpdateTierRequestAccessConditionsItem {
            balance_definition_id: self.balance_definition_id,
            minimum_value: self.minimum_value,
        })
    }
}
