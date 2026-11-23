pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponse {
    /// Maximum number of times a consumer can be attributed this reward
    #[serde(rename = "attributionPerConsumer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_per_consumer: Option<i64>,
    /// Unique identifier for the balance definition
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Unique code for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Total number of codes generated
    #[serde(rename = "codeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_count: Option<i64>,
    /// Unique identifier for the code generator
    #[serde(rename = "codeGeneratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_generator_id: Option<String>,
    /// Unique identifier for the code pool
    #[serde(rename = "codePoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_pool_id: Option<String>,
    /// Configuration settings for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Disabled date of the reward
    #[serde(rename = "disabledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub disabled_at: Option<DateTime<FixedOffset>>,
    /// End date of the reward validity
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Expiration date of the reward
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// Select startOfPeriod to configure rewards expiry on start of day/week/month/year. Select endOfPeriod to configure reward expiry on end of day/week/month/year, else select noModification
    #[serde(rename = "expirationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_modifier: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier>,
    /// Unit of time for the rewards's availability (e.g., day/week/month/year).
    #[serde(rename = "expirationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_unit: Option<String>,
    /// Number of days/weeks/month/year for reward expiry
    #[serde(rename = "expirationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_value: Option<i64>,
    /// object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator>,
    /// Unique identifier for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Attribution / Redeem Limits for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem>>,
    /// Id of the loyalty program to which the current reward belongs to
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Additional data for reward definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Name of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selected products for reward definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem>>,
    /// Public description for the reward
    #[serde(rename = "publicDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_description: Option<String>,
    /// Public Image for the reward
    #[serde(rename = "publicImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_image: Option<String>,
    /// Public name for the reward
    #[serde(rename = "publicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,
    /// Defines the redeem limit for the consumer
    #[serde(rename = "redeemPerConsumer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_per_consumer: Option<i64>,
    /// Rules defined to redeem a reward
    #[serde(rename = "redeemRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_rules: Option<Vec<String>>,
    /// object
    #[serde(rename = "rewardConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_configs: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs>,
    /// Rule to define the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRule>,
    /// Start date of attribution of the reward
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_date: Option<DateTime<FixedOffset>>,
    /// Id of the selected balance while redeeming / attributing a reward
    #[serde(rename = "subtractBalanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_balance_definition_id: Option<String>,
    /// Strategy of the Balance while redeeming / attributing a reward
    #[serde(rename = "subtractBalanceStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_balance_strategy: Option<String>,
    /// Amount of balance to be selected while redeeming / attributing a reward
    #[serde(rename = "subtractBalanceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_balance_value: Option<i64>,
    /// Value to indicate to subtract full balance or not
    #[serde(rename = "subtractTotalBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_total_balance: Option<bool>,
    /// Defines the limit to which a consumer can attribute a reward
    #[serde(rename = "totalAttribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_attribution: Option<i64>,
    /// Defines the limit to which a consumer can redeem a reward
    #[serde(rename = "totalRedeem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_redeem: Option<i64>,
    /// Id of the Rule to be updated for that reward
    #[serde(rename = "triggerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// Selected unit of the balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Timestamp for when this reward was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Value of metric in selected config for reward definition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
    /// Type of config selected for reward definition
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponse {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseBuilder {
    attribution_per_consumer: Option<i64>,
    balance_definition_id: Option<String>,
    code: Option<String>,
    code_count: Option<i64>,
    code_generator_id: Option<String>,
    code_pool_id: Option<String>,
    config: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    disabled_at: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
    expiration_date: Option<DateTime<FixedOffset>>,
    expiration_modifier: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier>,
    expiration_unit: Option<String>,
    expiration_value: Option<i64>,
    generator: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator>,
    id: Option<String>,
    limits: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem>>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    products: Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem>>,
    public_description: Option<String>,
    public_image: Option<String>,
    public_name: Option<String>,
    redeem_per_consumer: Option<i64>,
    redeem_rules: Option<Vec<String>>,
    reward_configs: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs>,
    rule: Option<GetLoyaltyOfferProgramsPidRewardsRidResponseRule>,
    start_date: Option<DateTime<FixedOffset>>,
    subtract_balance_definition_id: Option<String>,
    subtract_balance_strategy: Option<String>,
    subtract_balance_value: Option<i64>,
    subtract_total_balance: Option<bool>,
    total_attribution: Option<i64>,
    total_redeem: Option<i64>,
    trigger_id: Option<String>,
    unit: Option<String>,
    updated_at: Option<String>,
    value: Option<f64>,
    value_type: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseBuilder {
    pub fn attribution_per_consumer(mut self, value: i64) -> Self {
        self.attribution_per_consumer = Some(value);
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_count(mut self, value: i64) -> Self {
        self.code_count = Some(value);
        self
    }

    pub fn code_generator_id(mut self, value: impl Into<String>) -> Self {
        self.code_generator_id = Some(value.into());
        self
    }

    pub fn code_pool_id(mut self, value: impl Into<String>) -> Self {
        self.code_pool_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: impl Into<String>) -> Self {
        self.config = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn disabled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.disabled_at = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn expiration_modifier(
        mut self,
        value: GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier,
    ) -> Self {
        self.expiration_modifier = Some(value);
        self
    }

    pub fn expiration_unit(mut self, value: impl Into<String>) -> Self {
        self.expiration_unit = Some(value.into());
        self
    }

    pub fn expiration_value(mut self, value: i64) -> Self {
        self.expiration_value = Some(value);
        self
    }

    pub fn generator(
        mut self,
        value: GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator,
    ) -> Self {
        self.generator = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn limits(
        mut self,
        value: Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem>,
    ) -> Self {
        self.limits = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
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

    pub fn products(
        mut self,
        value: Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem>,
    ) -> Self {
        self.products = Some(value);
        self
    }

    pub fn public_description(mut self, value: impl Into<String>) -> Self {
        self.public_description = Some(value.into());
        self
    }

    pub fn public_image(mut self, value: impl Into<String>) -> Self {
        self.public_image = Some(value.into());
        self
    }

    pub fn public_name(mut self, value: impl Into<String>) -> Self {
        self.public_name = Some(value.into());
        self
    }

    pub fn redeem_per_consumer(mut self, value: i64) -> Self {
        self.redeem_per_consumer = Some(value);
        self
    }

    pub fn redeem_rules(mut self, value: Vec<String>) -> Self {
        self.redeem_rules = Some(value);
        self
    }

    pub fn reward_configs(
        mut self,
        value: GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs,
    ) -> Self {
        self.reward_configs = Some(value);
        self
    }

    pub fn rule(mut self, value: GetLoyaltyOfferProgramsPidRewardsRidResponseRule) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn subtract_balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.subtract_balance_definition_id = Some(value.into());
        self
    }

    pub fn subtract_balance_strategy(mut self, value: impl Into<String>) -> Self {
        self.subtract_balance_strategy = Some(value.into());
        self
    }

    pub fn subtract_balance_value(mut self, value: i64) -> Self {
        self.subtract_balance_value = Some(value);
        self
    }

    pub fn subtract_total_balance(mut self, value: bool) -> Self {
        self.subtract_total_balance = Some(value);
        self
    }

    pub fn total_attribution(mut self, value: i64) -> Self {
        self.total_attribution = Some(value);
        self
    }

    pub fn total_redeem(mut self, value: i64) -> Self {
        self.total_redeem = Some(value);
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_type(mut self, value: impl Into<String>) -> Self {
        self.value_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponse`].
    pub fn build(self) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponse, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponse {
            attribution_per_consumer: self.attribution_per_consumer,
            balance_definition_id: self.balance_definition_id,
            code: self.code,
            code_count: self.code_count,
            code_generator_id: self.code_generator_id,
            code_pool_id: self.code_pool_id,
            config: self.config,
            created_at: self.created_at,
            disabled_at: self.disabled_at,
            end_date: self.end_date,
            expiration_date: self.expiration_date,
            expiration_modifier: self.expiration_modifier,
            expiration_unit: self.expiration_unit,
            expiration_value: self.expiration_value,
            generator: self.generator,
            id: self.id,
            limits: self.limits,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            name: self.name,
            products: self.products,
            public_description: self.public_description,
            public_image: self.public_image,
            public_name: self.public_name,
            redeem_per_consumer: self.redeem_per_consumer,
            redeem_rules: self.redeem_rules,
            reward_configs: self.reward_configs,
            rule: self.rule,
            start_date: self.start_date,
            subtract_balance_definition_id: self.subtract_balance_definition_id,
            subtract_balance_strategy: self.subtract_balance_strategy,
            subtract_balance_value: self.subtract_balance_value,
            subtract_total_balance: self.subtract_total_balance,
            total_attribution: self.total_attribution,
            total_redeem: self.total_redeem,
            trigger_id: self.trigger_id,
            unit: self.unit,
            updated_at: self.updated_at,
            value: self.value,
            value_type: self.value_type,
        })
    }
}
