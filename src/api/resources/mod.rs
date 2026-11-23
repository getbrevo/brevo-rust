//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Account**
//! - **MasterAccount**
//! - **User**
//! - **Process**
//! - **Senders**
//! - **Domains**
//! - **Webhooks**
//! - **ExternalFeeds**
//! - **CustomObjects**
//! - **Contacts**
//! - **ConsentGroups**
//! - **Conversations**
//! - **Ecommerce**
//! - **Coupons**
//! - **Payments**
//! - **Event**
//! - **InboundParsing**
//! - **Balance**
//! - **Program**
//! - **Reward**
//! - **Tier**
//! - **Wallet**
//! - **EmailCampaigns**
//! - **SmsCampaigns**
//! - **WhatsAppCampaigns**
//! - **Companies**
//! - **Deals**
//! - **Files**
//! - **Notes**
//! - **Tasks**
//! - **TransactionalWhatsApp**
//! - **TransactionalEmails**
//! - **TransactionalSms**
//! - **SmsTemplates**
//! - **OAuth**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod balance;
pub mod companies;
pub mod consent_groups;
pub mod contacts;
pub mod conversations;
pub mod coupons;
pub mod custom_objects;
pub mod deals;
pub mod domains;
pub mod ecommerce;
pub mod email_campaigns;
pub mod event;
pub mod external_feeds;
pub mod files;
pub mod inbound_parsing;
pub mod master_account;
pub mod notes;
pub mod o_auth;
pub mod payments;
pub mod process;
pub mod program;
pub mod reward;
pub mod senders;
pub mod sms_campaigns;
pub mod sms_templates;
pub mod tasks;
pub mod tier;
pub mod transactional_emails;
pub mod transactional_sms;
pub mod transactional_whats_app;
pub mod user;
pub mod wallet;
pub mod webhooks;
pub mod whats_app_campaigns;
pub struct BrevoClient {
    pub config: ClientConfig,
    pub account: AccountClient,
    pub master_account: MasterAccountClient,
    pub user: UserClient,
    pub process: ProcessClient,
    pub senders: SendersClient,
    pub domains: DomainsClient,
    pub webhooks: WebhooksClient,
    pub external_feeds: ExternalFeedsClient,
    pub custom_objects: CustomObjectsClient,
    pub contacts: ContactsClient,
    pub consent_groups: ConsentGroupsClient,
    pub conversations: ConversationsClient,
    pub ecommerce: EcommerceClient,
    pub coupons: CouponsClient,
    pub payments: PaymentsClient,
    pub event: EventClient,
    pub inbound_parsing: InboundParsingClient,
    pub balance: BalanceClient,
    pub program: ProgramClient,
    pub reward: RewardClient,
    pub tier: TierClient,
    pub wallet: WalletClient,
    pub email_campaigns: EmailCampaignsClient,
    pub sms_campaigns: SmsCampaignsClient,
    pub whats_app_campaigns: WhatsAppCampaignsClient,
    pub companies: CompaniesClient,
    pub deals: DealsClient,
    pub files: FilesClient,
    pub notes: NotesClient,
    pub tasks: TasksClient,
    pub transactional_whats_app: TransactionalWhatsAppClient,
    pub transactional_emails: TransactionalEmailsClient,
    pub transactional_sms: TransactionalSmsClient,
    pub sms_templates: SmsTemplatesClient,
    pub o_auth: OAuthClient,
}

impl BrevoClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            account: AccountClient::new(config.clone())?,
            master_account: MasterAccountClient::new(config.clone())?,
            user: UserClient::new(config.clone())?,
            process: ProcessClient::new(config.clone())?,
            senders: SendersClient::new(config.clone())?,
            domains: DomainsClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            external_feeds: ExternalFeedsClient::new(config.clone())?,
            custom_objects: CustomObjectsClient::new(config.clone())?,
            contacts: ContactsClient::new(config.clone())?,
            consent_groups: ConsentGroupsClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            ecommerce: EcommerceClient::new(config.clone())?,
            coupons: CouponsClient::new(config.clone())?,
            payments: PaymentsClient::new(config.clone())?,
            event: EventClient::new(config.clone())?,
            inbound_parsing: InboundParsingClient::new(config.clone())?,
            balance: BalanceClient::new(config.clone())?,
            program: ProgramClient::new(config.clone())?,
            reward: RewardClient::new(config.clone())?,
            tier: TierClient::new(config.clone())?,
            wallet: WalletClient::new(config.clone())?,
            email_campaigns: EmailCampaignsClient::new(config.clone())?,
            sms_campaigns: SmsCampaignsClient::new(config.clone())?,
            whats_app_campaigns: WhatsAppCampaignsClient::new(config.clone())?,
            companies: CompaniesClient::new(config.clone())?,
            deals: DealsClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            notes: NotesClient::new(config.clone())?,
            tasks: TasksClient::new(config.clone())?,
            transactional_whats_app: TransactionalWhatsAppClient::new(config.clone())?,
            transactional_emails: TransactionalEmailsClient::new(config.clone())?,
            transactional_sms: TransactionalSmsClient::new(config.clone())?,
            sms_templates: SmsTemplatesClient::new(config.clone())?,
            o_auth: OAuthClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use balance::BalanceClient;
pub use companies::CompaniesClient;
pub use consent_groups::ConsentGroupsClient;
pub use contacts::ContactsClient;
pub use conversations::ConversationsClient;
pub use coupons::CouponsClient;
pub use custom_objects::CustomObjectsClient;
pub use deals::DealsClient;
pub use domains::DomainsClient;
pub use ecommerce::EcommerceClient;
pub use email_campaigns::EmailCampaignsClient;
pub use event::EventClient;
pub use external_feeds::ExternalFeedsClient;
pub use files::FilesClient;
pub use inbound_parsing::InboundParsingClient;
pub use master_account::MasterAccountClient;
pub use notes::NotesClient;
pub use o_auth::OAuthClient;
pub use payments::PaymentsClient;
pub use process::ProcessClient;
pub use program::ProgramClient;
pub use reward::RewardClient;
pub use senders::SendersClient;
pub use sms_campaigns::SmsCampaignsClient;
pub use sms_templates::SmsTemplatesClient;
pub use tasks::TasksClient;
pub use tier::TierClient;
pub use transactional_emails::TransactionalEmailsClient;
pub use transactional_sms::TransactionalSmsClient;
pub use transactional_whats_app::TransactionalWhatsAppClient;
pub use user::UserClient;
pub use wallet::WalletClient;
pub use webhooks::WebhooksClient;
pub use whats_app_campaigns::WhatsAppCampaignsClient;
