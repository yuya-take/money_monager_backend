use std::sync::Arc;

use derive_new::new;

use money_manager_adapter::modules::RepositoriesModuleExt;
use money_manager_domain::repository::health_check::HealthCheckRepository;

#[derive(new)]
pub struct HealthCheckUseCase<R: RepositoriesModuleExt> {
    repository: Arc<R>,
}

impl<R: RepositoriesModuleExt> HealthCheckUseCase<R> {
    pub async fn diagnose_dynamodb_conn(&self) -> anyhow::Result<()> {
        let repository = self.repository.health_check_repository();
        repository.check_dynamodb_conn().await
    }
}
