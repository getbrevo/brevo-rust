pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidOffersResponseItemsItem {
    /// Timestamp for when this reward was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp for when this reward attribution ended
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub end_date: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Loyalty Program to which current reward is associated
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Name of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public image of the reward
    #[serde(rename = "publicImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_image: Option<String>,
    /// Timestamp for when this reward attribution started
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_date: Option<DateTime<FixedOffset>>,
    /// State of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Timestamp for when this reward attribution was updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidOffersResponseItemsItem {
    pub fn builder() -> GetLoyaltyOfferProgramsPidOffersResponseItemsItemBuilder {
        <GetLoyaltyOfferProgramsPidOffersResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidOffersResponseItemsItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    name: Option<String>,
    public_image: Option<String>,
    start_date: Option<DateTime<FixedOffset>>,
    state: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidOffersResponseItemsItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn public_image(mut self, value: impl Into<String>) -> Self {
        self.public_image = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidOffersResponseItemsItem`].
    pub fn build(self) -> Result<GetLoyaltyOfferProgramsPidOffersResponseItemsItem, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidOffersResponseItemsItem {
            created_at: self.created_at,
            end_date: self.end_date,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            name: self.name,
            public_image: self.public_image,
            start_date: self.start_date,
            state: self.state,
            updated_at: self.updated_at,
        })
    }
}
