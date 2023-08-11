use std::sync::Arc;

use async_trait::async_trait;
use bitcoin::{secp256k1::PublicKey, Address};
use cashu_crab::{lightning_invoice::Bolt11Invoice, Amount, Sha256};
use cln_rpc::model::responses::ListinvoicesInvoicesStatus;
use gl_client::pb::cln::listinvoices_invoices::ListinvoicesInvoicesStatus as GL_ListInvoiceStatus;
use serde::{Deserialize, Serialize};

pub use error::Error;
use node_manager_types::{requests, responses, Bolt11};

pub mod cln;
pub mod error;
pub mod greenlight;
pub mod jwt_auth;
pub mod ldk;
pub mod lnurl;
pub mod node_manager;

#[derive(Clone)]
pub struct Ln {
    pub ln_processor: Arc<dyn LnProcessor>,
    pub node_manager: Option<node_manager::Nodemanger>,
}

/// Possible states of an invoice
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum InvoiceStatus {
    Unpaid,
    Paid,
    Expired,
    InFlight,
}

impl From<ListinvoicesInvoicesStatus> for InvoiceStatus {
    fn from(status: ListinvoicesInvoicesStatus) -> Self {
        match status {
            ListinvoicesInvoicesStatus::UNPAID => Self::Unpaid,
            ListinvoicesInvoicesStatus::PAID => Self::Paid,
            ListinvoicesInvoicesStatus::EXPIRED => Self::Expired,
        }
    }
}

impl From<GL_ListInvoiceStatus> for InvoiceStatus {
    fn from(status: GL_ListInvoiceStatus) -> Self {
        match status {
            GL_ListInvoiceStatus::Unpaid => Self::Unpaid,
            GL_ListInvoiceStatus::Paid => Self::Paid,
            GL_ListInvoiceStatus::Expired => Self::Expired,
        }
    }
}

impl From<ldk_node::PaymentStatus> for InvoiceStatus {
    fn from(status: ldk_node::PaymentStatus) -> Self {
        match status {
            ldk_node::PaymentStatus::Pending => Self::Unpaid,
            ldk_node::PaymentStatus::Succeeded => Self::Paid,
            ldk_node::PaymentStatus::Failed => Self::Expired,
        }
    }
}

impl From<cashu_crab::types::InvoiceStatus> for InvoiceStatus {
    fn from(status: cashu_crab::types::InvoiceStatus) -> Self {
        match status {
            cashu_crab::types::InvoiceStatus::Unpaid => Self::Unpaid,
            cashu_crab::types::InvoiceStatus::Paid => Self::Paid,
            cashu_crab::types::InvoiceStatus::Expired => Self::Expired,
            cashu_crab::types::InvoiceStatus::InFlight => Self::InFlight,
        }
    }
}

pub fn cashu_crab_invoice(invoice: InvoiceStatus) -> cashu_crab::types::InvoiceStatus {
    match invoice {
        InvoiceStatus::Unpaid => cashu_crab::types::InvoiceStatus::Unpaid,
        InvoiceStatus::Paid => cashu_crab::types::InvoiceStatus::Paid,
        InvoiceStatus::Expired => cashu_crab::types::InvoiceStatus::Expired,
        InvoiceStatus::InFlight => cashu_crab::types::InvoiceStatus::InFlight,
    }
}

impl ToString for InvoiceStatus {
    fn to_string(&self) -> String {
        match self {
            InvoiceStatus::Paid => "Paid".to_string(),
            InvoiceStatus::Unpaid => "Unpaid".to_string(),
            InvoiceStatus::Expired => "Expired".to_string(),
            InvoiceStatus::InFlight => "InFlight".to_string(),
        }
    }
}

/// Possible states of an invoice
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InvoiceTokenStatus {
    Issued,
    NotIssued,
}

/// Invoice information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceInfo {
    /// Payment hash of LN Invoice
    pub payment_hash: Sha256,
    /// random hash generated by the mint to internally look up the invoice state
    pub hash: Sha256,
    pub invoice: Bolt11Invoice,
    pub amount: Amount,
    pub status: InvoiceStatus,
    pub token_status: InvoiceTokenStatus,
    pub memo: String,
    pub confirmed_at: Option<u64>,
}

impl InvoiceInfo {
    pub fn new(
        payment_hash: Sha256,
        hash: Sha256,
        invoice: Bolt11Invoice,
        amount: Amount,
        status: InvoiceStatus,
        memo: &str,
        confirmed_at: Option<u64>,
    ) -> Self {
        Self {
            payment_hash,
            hash,
            invoice,
            amount,
            status,
            token_status: InvoiceTokenStatus::NotIssued,
            memo: memo.to_string(),
            confirmed_at,
        }
    }

    pub fn as_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }
}

#[async_trait]
pub trait LnProcessor: Send + Sync {
    async fn get_invoice(
        &self,
        amount: Amount,
        hash: Sha256,
        description: &str,
    ) -> Result<InvoiceInfo, Error>;

    async fn wait_invoice(&self) -> Result<(), Error>;

    async fn pay_invoice(
        &self,
        invoice: Bolt11Invoice,
        max_fee: Option<Amount>,
    ) -> Result<(String, Amount), Error>;

    async fn check_invoice_status(&self, payment_hash: &Sha256) -> Result<InvoiceStatus, Error>;
}

#[async_trait]
pub trait LnNodeManager: Send + Sync {
    async fn new_onchain_address(&self) -> Result<Address, Error>;

    async fn open_channel(
        &self,
        open_channel_request: requests::OpenChannelRequest,
    ) -> Result<String, Error>;

    async fn list_channels(&self) -> Result<Vec<responses::ChannelInfo>, Error>;

    async fn get_balance(&self) -> Result<responses::BalanceResponse, Error>;

    async fn pay_invoice(&self, bolt11: Bolt11) -> Result<responses::PayInvoiceResponse, Error>;

    async fn create_invoice(
        &self,
        amount: Amount,
        description: String,
    ) -> Result<Bolt11Invoice, Error>;

    async fn pay_on_chain(&self, address: Address, amount: Amount) -> Result<String, Error>;

    async fn close(&self, channel_id: String, peer_id: Option<PublicKey>) -> Result<(), Error>;

    async fn pay_keysend(&self, destination: PublicKey, amount: Amount) -> Result<String, Error>;

    async fn connect_peer(
        &self,
        public_key: PublicKey,
        host: String,
        port: u16,
    ) -> Result<responses::PeerInfo, Error>;

    async fn list_peers(&self) -> Result<Vec<responses::PeerInfo>, Error>;
}
