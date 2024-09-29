use std::sync::Arc;

use money_manager_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::dynamodb::DynamoDB,
};
use money_manager_app::usecase::health_check::HealthCheckUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule> {
        &self.health_check_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let dynamo_db = DynamoDB::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(dynamo_db));
        let health_check_use_case = HealthCheckUseCase::new(repositories_module.clone());
        Self {
            health_check_use_case,
        }
    }
}
