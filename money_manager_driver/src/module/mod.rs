use std::sync::Arc;

use money_manager_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::dynamodb::{init_client, DynamoDB},
};
use money_manager_app::usecase::{health_check::HealthCheckUseCase, user::UserUseCase};

pub struct Modules {
    health_check_use_case: HealthCheckUseCase<RepositoriesModule>,
    user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule>;
    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule> {
        &self.health_check_use_case
    }
    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let client = init_client().await;
        let dynamo_db = DynamoDB::new(client);

        let repositories_module = Arc::new(RepositoriesModule::new(dynamo_db));
        let health_check_use_case = HealthCheckUseCase::new(repositories_module.clone());
        let user_use_case = UserUseCase::new(repositories_module.clone());
        Self {
            health_check_use_case,
            user_use_case,
        }
    }
}
