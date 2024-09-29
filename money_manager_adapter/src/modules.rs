use money_manager_domain::repository::health_check::HealthCheckRepository;

use crate::{persistence::dynamodb::DynamoDB, repository::health_check::HealthCheckRepositoryImpl};

pub struct RepositoriesModule {
    health_check_repository: HealthCheckRepositoryImpl,
}

pub trait RepositoriesModuleExt {
    type HealthCheckRepo: HealthCheckRepository;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type HealthCheckRepo = HealthCheckRepositoryImpl;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo {
        &self.health_check_repository
    }
}

impl RepositoriesModule {
    pub fn new(dynamo_db: DynamoDB) -> Self {
        Self {
            health_check_repository: HealthCheckRepositoryImpl::new(dynamo_db.clone()),
        }
    }
}
