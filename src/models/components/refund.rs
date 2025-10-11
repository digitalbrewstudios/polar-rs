use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{refund_reason::RefundReason, refund_status::RefundStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub status: RefundStatus,
    pub reason: RefundReason,
    pub amount: i64,
    pub tax_amount: i64,
    pub currency: String,
    pub organization_id: String,
    pub order_id: String,
    pub subscription_id: Option<String>,
    pub checkout_id: Option<String>,
    pub customer_id: String,
    pub revoke_benfits: bool,
}
