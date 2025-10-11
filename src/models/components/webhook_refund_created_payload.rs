use super::refund::Refund;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRefundCreatedPayload {
    #[serde(rename = "type")]
    pub timestamp: DateTime<Utc>,
    pub data: Refund,
}
