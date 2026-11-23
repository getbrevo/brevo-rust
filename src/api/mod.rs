//! API client and types for the Brevo API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AccountClient, BalanceClient, BrevoClient, CompaniesClient, ConsentGroupsClient,
    ContactsClient, ConversationsClient, CouponsClient, CustomObjectsClient, DealsClient,
    DomainsClient, EcommerceClient, EmailCampaignsClient, EventClient, ExternalFeedsClient,
    FilesClient, InboundParsingClient, MasterAccountClient, NotesClient, OAuthClient,
    PaymentsClient, ProcessClient, ProgramClient, RewardClient, SendersClient, SmsCampaignsClient,
    SmsTemplatesClient, TasksClient, TierClient, TransactionalEmailsClient, TransactionalSmsClient,
    TransactionalWhatsAppClient, UserClient, WalletClient, WebhooksClient, WhatsAppCampaignsClient,
};
pub use types::*;
