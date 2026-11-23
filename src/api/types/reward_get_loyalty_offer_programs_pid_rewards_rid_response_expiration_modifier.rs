pub use crate::prelude::*;

/// Select startOfPeriod to configure rewards expiry on start of day/week/month/year. Select endOfPeriod to configure reward expiry on end of day/week/month/year, else select noModification
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    StartOfPeriod,
    EndOfPeriod,
    NoModification,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StartOfPeriod => serializer.serialize_str("startOfPeriod"),
            Self::EndOfPeriod => serializer.serialize_str("endOfPeriod"),
            Self::NoModification => serializer.serialize_str("noModification"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "startOfPeriod" => Ok(Self::StartOfPeriod),
            "endOfPeriod" => Ok(Self::EndOfPeriod),
            "noModification" => Ok(Self::NoModification),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetLoyaltyOfferProgramsPidRewardsRidResponseExpirationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "startOfPeriod"),
            Self::EndOfPeriod => write!(f, "endOfPeriod"),
            Self::NoModification => write!(f, "noModification"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
