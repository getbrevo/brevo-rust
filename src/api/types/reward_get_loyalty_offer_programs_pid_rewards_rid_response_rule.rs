pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRule {
    /// Selected rule condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<RuleConditionResponse>,
    /// Timestamp when the rule was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Description of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Selected event in the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent>,
    /// Metric to identify if it's an internal rule or not
    #[serde(rename = "isInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Loyalty Program id to which current rule is associated
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Loyalty Version id to which current rule is associated
    #[serde(rename = "loyaltyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_version_id: Option<i64>,
    /// Additional data to define the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Rule name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Results of the rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem>>,
    /// Unique identifier for the rule
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// Type of the rule
    #[serde(rename = "ruleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// Timestamp when the rule was updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRule {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseRuleBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleBuilder {
    condition: Option<RuleConditionResponse>,
    created_at: Option<String>,
    description: Option<String>,
    event: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent>,
    is_internal: Option<bool>,
    loyalty_program_id: Option<String>,
    loyalty_version_id: Option<i64>,
    meta: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    results: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem>>,
    rule_id: Option<String>,
    rule_type: Option<String>,
    updated_at: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleBuilder {
    pub fn condition(mut self, value: RuleConditionResponse) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn is_internal(mut self, value: bool) -> Self {
        self.is_internal = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn loyalty_version_id(mut self, value: i64) -> Self {
        self.loyalty_version_id = Some(value);
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn results(
        mut self,
        value: Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem>,
    ) -> Self {
        self.results = Some(value);
        self
    }

    pub fn rule_id(mut self, value: impl Into<String>) -> Self {
        self.rule_id = Some(value.into());
        self
    }

    pub fn rule_type(mut self, value: impl Into<String>) -> Self {
        self.rule_type = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseRule`].
    pub fn build(self) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseRule, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseRule {
            condition: self.condition,
            created_at: self.created_at,
            description: self.description,
            event: self.event,
            is_internal: self.is_internal,
            loyalty_program_id: self.loyalty_program_id,
            loyalty_version_id: self.loyalty_version_id,
            meta: self.meta,
            name: self.name,
            results: self.results,
            rule_id: self.rule_id,
            rule_type: self.rule_type,
            updated_at: self.updated_at,
        })
    }
}
