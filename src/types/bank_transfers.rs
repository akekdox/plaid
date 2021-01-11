use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BankResponse {
    pub bank_transfer: BankTransfer,

    /// A unique identifier for the request, which can be used for
    /// troubleshooting. This identifier, like all Plaid identifiers, is case
    /// sensitive.
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BankTransfer {
    pub id: String,
    pub ach_class: String,
    pub account_id: String,
    pub Type: String,
    pub user: String,
}