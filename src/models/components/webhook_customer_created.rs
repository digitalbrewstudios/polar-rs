use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCustomerCreatedPayload {
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub timestamp: DateTime<Utc>,
    pub data: CustomerCreatedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedData {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub external_id: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub billing_address: Option<Value>,
    pub tax_id: Option<Vec<Value>>,
    pub organization_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
    pub avatar_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_customer_updated_payload_from_json() {
        let body_json = r#"
{
    "type": "customer.created",
    "timestamp": "2025-11-01T22:51:31.323069Z",
    "data": {
        "id": "17926f0f-5541-41b1-a3bd-65af6b9b8c34",
        "created_at": "2025-11-01T22:51:31.216868Z",
        "modified_at": null,
        "metadata": {
            "feet": "yes"
        },
        "external_id": "gothbaddie",
        "email": "qy7u9458ghq890q4ytnvfkzvaheauiph@gmail.com",
        "email_verified": false,
        "name": "Goth Baddie McGee",
        "billing_address": null,
        "tax_id": null,
        "organization_id": "7eae7216-658f-4efd-afe3-1c6ad07d58e5",
        "deleted_at": null,
        "avatar_url": "https://www.gravatar.com/avatar/115039e1011a18a7e28819c6f9d9ac0991e7d8df084730be52e26c2cd0d8853e?d=404"
    }
}    "#;

        let body: WebhookCustomerCreatedPayload = serde_json::from_str(body_json).unwrap();
        println!("{body:?}")
    }
}
