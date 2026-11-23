pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SendWhatsappMessageRequest {
    SendWhatsappMessageRequestParams(SendWhatsappMessageRequestParams),

    SendWhatsappMessageRequestText(SendWhatsappMessageRequestText),
}

impl SendWhatsappMessageRequest {
    pub fn is_send_whatsapp_message_request_params(&self) -> bool {
        matches!(self, Self::SendWhatsappMessageRequestParams(_))
    }

    pub fn is_send_whatsapp_message_request_text(&self) -> bool {
        matches!(self, Self::SendWhatsappMessageRequestText(_))
    }

    pub fn as_send_whatsapp_message_request_params(
        &self,
    ) -> Option<&SendWhatsappMessageRequestParams> {
        match self {
            Self::SendWhatsappMessageRequestParams(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_send_whatsapp_message_request_params(
        self,
    ) -> Option<SendWhatsappMessageRequestParams> {
        match self {
            Self::SendWhatsappMessageRequestParams(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_send_whatsapp_message_request_text(&self) -> Option<&SendWhatsappMessageRequestText> {
        match self {
            Self::SendWhatsappMessageRequestText(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_send_whatsapp_message_request_text(self) -> Option<SendWhatsappMessageRequestText> {
        match self {
            Self::SendWhatsappMessageRequestText(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SendWhatsappMessageRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendWhatsappMessageRequestParams(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SendWhatsappMessageRequestText(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
