pub use crate::prelude::*;

/// Modifier for the duration.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTierGroupRequestUpgradeScheduleDurationModifier {
    StartOfPeriod,
    EndOfPeriod,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTierGroupRequestUpgradeScheduleDurationModifier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StartOfPeriod => serializer.serialize_str("start_of_period"),
            Self::EndOfPeriod => serializer.serialize_str("end_of_period"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTierGroupRequestUpgradeScheduleDurationModifier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "start_of_period" => Ok(Self::StartOfPeriod),
            "end_of_period" => Ok(Self::EndOfPeriod),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTierGroupRequestUpgradeScheduleDurationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartOfPeriod => write!(f, "start_of_period"),
            Self::EndOfPeriod => write!(f, "end_of_period"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
