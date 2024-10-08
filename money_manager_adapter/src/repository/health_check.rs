use async_trait::async_trait;
use std::sync::Arc;

use money_manager_domain::repository::health_check::HealthCheckRepository;

use crate::persistence::dynamodb::DynamoDB;

pub struct HealthCheckRepositoryImpl {
    dynamo_db: Arc<DynamoDB>,
}

impl HealthCheckRepositoryImpl {
    pub fn new(dynamo_db: DynamoDB) -> Self {
        Self {
            dynamo_db: Arc::new(dynamo_db),
        }
    }
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_dynamodb_conn(&self) -> anyhow::Result<()> {
        let _ = self.dynamo_db.client.list_tables().send().await?;
        Ok(())
    }
}
