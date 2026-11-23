pub use crate::prelude::*;

/// Error code displayed in case of a failure
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotFoundErrorBodyCode {
    InvalidParameter,
    MissingParameter,
    OutOfRange,
    CampaignProcessing,
    CampaignSent,
    DocumentNotFound,
    NotEnoughCredits,
    PermissionDenied,
    DuplicateParameter,
    DuplicateRequest,
    MethodNotAllowed,
    Unauthorized,
    AccountUnderValidation,
    NotAcceptable,
    BadRequest,
    UnprocessableEntity,
    DomainDoesNotExist,
    ContactEmailNotFound,
    AttributeNotFound,
    CategoryIdNotFound,
    InvalidParametersPassed,
    RecordSForIdentifierNotFound,
    ReturnedWhenQueryParamsAreInvalid,
    ReturnedWhenInvalidDataPosted,
    FeedNotFound,
    CampaignIdNotFound,
    ApiKeyNotFound,
    DmarcPolicyRequiresDomainAuthentication,
    DnsRecordsNotProperlyConfigured,
    InvalidOtpCodeProvided,
    OtpCodeHasExpired,
    DomainAlreadyExistsInYourAccount,
    TheSumOfAllIpWeightsMustEqual100,
    AuthenticationFailed,
    InsufficientCredits,
    RequestAlreadyProcessed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotFoundErrorBodyCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InvalidParameter => serializer.serialize_str("invalid_parameter"),
            Self::MissingParameter => serializer.serialize_str("missing_parameter"),
            Self::OutOfRange => serializer.serialize_str("out_of_range"),
            Self::CampaignProcessing => serializer.serialize_str("campaign_processing"),
            Self::CampaignSent => serializer.serialize_str("campaign_sent"),
            Self::DocumentNotFound => serializer.serialize_str("document_not_found"),
            Self::NotEnoughCredits => serializer.serialize_str("not_enough_credits"),
            Self::PermissionDenied => serializer.serialize_str("permission_denied"),
            Self::DuplicateParameter => serializer.serialize_str("duplicate_parameter"),
            Self::DuplicateRequest => serializer.serialize_str("duplicate_request"),
            Self::MethodNotAllowed => serializer.serialize_str("method_not_allowed"),
            Self::Unauthorized => serializer.serialize_str("unauthorized"),
            Self::AccountUnderValidation => serializer.serialize_str("account_under_validation"),
            Self::NotAcceptable => serializer.serialize_str("not_acceptable"),
            Self::BadRequest => serializer.serialize_str("bad_request"),
            Self::UnprocessableEntity => serializer.serialize_str("unprocessable_entity"),
            Self::DomainDoesNotExist => serializer.serialize_str("Domain does not exist"),
            Self::ContactEmailNotFound => serializer.serialize_str("Contact email not found"),
            Self::AttributeNotFound => serializer.serialize_str("Attribute not found"),
            Self::CategoryIdNotFound => serializer.serialize_str("Category id not found"),
            Self::InvalidParametersPassed => serializer.serialize_str("Invalid parameters passed"),
            Self::RecordSForIdentifierNotFound => {
                serializer.serialize_str("Record(s) for identifier not found")
            }
            Self::ReturnedWhenQueryParamsAreInvalid => {
                serializer.serialize_str("Returned when query params are invalid")
            }
            Self::ReturnedWhenInvalidDataPosted => {
                serializer.serialize_str("Returned when invalid data posted")
            }
            Self::FeedNotFound => serializer.serialize_str("Feed not found"),
            Self::CampaignIdNotFound => serializer.serialize_str("Campaign ID not found"),
            Self::ApiKeyNotFound => serializer.serialize_str("api-key not found"),
            Self::DmarcPolicyRequiresDomainAuthentication => {
                serializer.serialize_str("DMARC policy requires domain authentication")
            }
            Self::DnsRecordsNotProperlyConfigured => {
                serializer.serialize_str("DNS records not properly configured")
            }
            Self::InvalidOtpCodeProvided => serializer.serialize_str("Invalid OTP code provided"),
            Self::OtpCodeHasExpired => serializer.serialize_str("OTP code has expired"),
            Self::DomainAlreadyExistsInYourAccount => {
                serializer.serialize_str("Domain already exists in your account")
            }
            Self::TheSumOfAllIpWeightsMustEqual100 => {
                serializer.serialize_str("The sum of all IP weights must equal 100")
            }
            Self::AuthenticationFailed => serializer.serialize_str("Authentication failed"),
            Self::InsufficientCredits => serializer.serialize_str("Insufficient credits"),
            Self::RequestAlreadyProcessed => serializer.serialize_str("Request already processed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotFoundErrorBodyCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid_parameter" => Ok(Self::InvalidParameter),
            "missing_parameter" => Ok(Self::MissingParameter),
            "out_of_range" => Ok(Self::OutOfRange),
            "campaign_processing" => Ok(Self::CampaignProcessing),
            "campaign_sent" => Ok(Self::CampaignSent),
            "document_not_found" => Ok(Self::DocumentNotFound),
            "not_enough_credits" => Ok(Self::NotEnoughCredits),
            "permission_denied" => Ok(Self::PermissionDenied),
            "duplicate_parameter" => Ok(Self::DuplicateParameter),
            "duplicate_request" => Ok(Self::DuplicateRequest),
            "method_not_allowed" => Ok(Self::MethodNotAllowed),
            "unauthorized" => Ok(Self::Unauthorized),
            "account_under_validation" => Ok(Self::AccountUnderValidation),
            "not_acceptable" => Ok(Self::NotAcceptable),
            "bad_request" => Ok(Self::BadRequest),
            "unprocessable_entity" => Ok(Self::UnprocessableEntity),
            "Domain does not exist" => Ok(Self::DomainDoesNotExist),
            "Contact email not found" => Ok(Self::ContactEmailNotFound),
            "Attribute not found" => Ok(Self::AttributeNotFound),
            "Category id not found" => Ok(Self::CategoryIdNotFound),
            "Invalid parameters passed" => Ok(Self::InvalidParametersPassed),
            "Record(s) for identifier not found" => Ok(Self::RecordSForIdentifierNotFound),
            "Returned when query params are invalid" => Ok(Self::ReturnedWhenQueryParamsAreInvalid),
            "Returned when invalid data posted" => Ok(Self::ReturnedWhenInvalidDataPosted),
            "Feed not found" => Ok(Self::FeedNotFound),
            "Campaign ID not found" => Ok(Self::CampaignIdNotFound),
            "api-key not found" => Ok(Self::ApiKeyNotFound),
            "DMARC policy requires domain authentication" => {
                Ok(Self::DmarcPolicyRequiresDomainAuthentication)
            }
            "DNS records not properly configured" => Ok(Self::DnsRecordsNotProperlyConfigured),
            "Invalid OTP code provided" => Ok(Self::InvalidOtpCodeProvided),
            "OTP code has expired" => Ok(Self::OtpCodeHasExpired),
            "Domain already exists in your account" => Ok(Self::DomainAlreadyExistsInYourAccount),
            "The sum of all IP weights must equal 100" => {
                Ok(Self::TheSumOfAllIpWeightsMustEqual100)
            }
            "Authentication failed" => Ok(Self::AuthenticationFailed),
            "Insufficient credits" => Ok(Self::InsufficientCredits),
            "Request already processed" => Ok(Self::RequestAlreadyProcessed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotFoundErrorBodyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter => write!(f, "invalid_parameter"),
            Self::MissingParameter => write!(f, "missing_parameter"),
            Self::OutOfRange => write!(f, "out_of_range"),
            Self::CampaignProcessing => write!(f, "campaign_processing"),
            Self::CampaignSent => write!(f, "campaign_sent"),
            Self::DocumentNotFound => write!(f, "document_not_found"),
            Self::NotEnoughCredits => write!(f, "not_enough_credits"),
            Self::PermissionDenied => write!(f, "permission_denied"),
            Self::DuplicateParameter => write!(f, "duplicate_parameter"),
            Self::DuplicateRequest => write!(f, "duplicate_request"),
            Self::MethodNotAllowed => write!(f, "method_not_allowed"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::AccountUnderValidation => write!(f, "account_under_validation"),
            Self::NotAcceptable => write!(f, "not_acceptable"),
            Self::BadRequest => write!(f, "bad_request"),
            Self::UnprocessableEntity => write!(f, "unprocessable_entity"),
            Self::DomainDoesNotExist => write!(f, "Domain does not exist"),
            Self::ContactEmailNotFound => write!(f, "Contact email not found"),
            Self::AttributeNotFound => write!(f, "Attribute not found"),
            Self::CategoryIdNotFound => write!(f, "Category id not found"),
            Self::InvalidParametersPassed => write!(f, "Invalid parameters passed"),
            Self::RecordSForIdentifierNotFound => write!(f, "Record(s) for identifier not found"),
            Self::ReturnedWhenQueryParamsAreInvalid => {
                write!(f, "Returned when query params are invalid")
            }
            Self::ReturnedWhenInvalidDataPosted => write!(f, "Returned when invalid data posted"),
            Self::FeedNotFound => write!(f, "Feed not found"),
            Self::CampaignIdNotFound => write!(f, "Campaign ID not found"),
            Self::ApiKeyNotFound => write!(f, "api-key not found"),
            Self::DmarcPolicyRequiresDomainAuthentication => {
                write!(f, "DMARC policy requires domain authentication")
            }
            Self::DnsRecordsNotProperlyConfigured => {
                write!(f, "DNS records not properly configured")
            }
            Self::InvalidOtpCodeProvided => write!(f, "Invalid OTP code provided"),
            Self::OtpCodeHasExpired => write!(f, "OTP code has expired"),
            Self::DomainAlreadyExistsInYourAccount => {
                write!(f, "Domain already exists in your account")
            }
            Self::TheSumOfAllIpWeightsMustEqual100 => {
                write!(f, "The sum of all IP weights must equal 100")
            }
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
            Self::InsufficientCredits => write!(f, "Insufficient credits"),
            Self::RequestAlreadyProcessed => write!(f, "Request already processed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
