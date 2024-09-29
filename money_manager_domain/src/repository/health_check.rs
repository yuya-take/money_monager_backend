use async_trait::async_trait;

// ドメイン層でトレイトを定義
#[async_trait]
pub trait HealthCheckRepository: Sync + Send {
    async fn check_dynamodb_conn(&self) -> anyhow::Result<()>;
}
