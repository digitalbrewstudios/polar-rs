use super::polar::Polar;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub struct Products<'a> {
    client: &'a Polar,
}

#[derive(Debug, Serialize)]
pub struct ListProductsQueryParams {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub query: Option<String>,
    pub is_archived: Option<bool>,
    pub is_recurring: Option<bool>,
    pub benefit_id: Option<Uuid>,
    pub page: i64,
    pub limit: i64,
    pub sorting: Option<String>, // TODO this is an enum
    pub metadata: Option<Value>,
}

impl Default for ListProductsQueryParams {
    fn default() -> Self {
        ListProductsQueryParams {
            id: None,
            organization_id: None,
            query: None,
            is_archived: None,
            is_recurring: None,
            benefit_id: None,
            page: 1,
            limit: 10,
            sorting: None,
            metadata: None,
        }
    }
}

impl<'a> Products<'a> {
    pub async fn get_product(&self, id: String) {
        let url = self
            .client
            .server
            .url()
            .join("v1/products")
            .unwrap()
            .join(&id)
            .unwrap();

        let response = self.client.client.get(url).send().await.unwrap();
        println!("{response:?}");
        let body = response.text().await.unwrap();
        println!("{body:?}");
    }
    pub async fn list_products(&self, params: Option<ListProductsQueryParams>) {
        let url = self.client.server.url().join("v1/products").unwrap();

        let response = self
            .client
            .client
            .get(url)
            .query(&params)
            .send()
            .await
            .unwrap();
        println!("{response:?}");
        let body = response.text().await.unwrap();
        println!("{body:?}");
    }

    pub async fn create_product(&self) {
        todo!()
    }

    pub async fn update_product(&self) {
        todo!()
    }

    pub async fn update_product_benefits(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::env;

    #[tokio::test]
    async fn test_list_products() {
        let client = client::polar::Polar::new(
            &env::var("POLAR_SANDBOX_API_KEY").unwrap(),
            client::polar::Server::Sandbox,
        );
        let products = Products { client: &client };
        products.list_products(None).await;
    }
}
