use money_manager_domain::{
    model::user::User,
    repository::{health_check::HealthCheckRepository, user::UserRepository},
};

use crate::{
    persistence::dynamodb::DynamoDB,
    repository::{health_check::HealthCheckRepositoryImpl, DynamoDBRepositoryImpl},
};

pub struct RepositoriesModule {
    health_check_repository: HealthCheckRepositoryImpl,
    user_repository: DynamoDBRepositoryImpl<User>,
}

pub trait RepositoriesModuleExt {
    type HealthCheckRepo: HealthCheckRepository;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo;

    type UserRepo: UserRepository;
    fn user_repository(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type HealthCheckRepo = HealthCheckRepositoryImpl;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo {
        &self.health_check_repository
    }

    type UserRepo = DynamoDBRepositoryImpl<User>;
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(dynamo_db: DynamoDB) -> Self {
        RepositoriesModule {
            health_check_repository: HealthCheckRepositoryImpl::new(dynamo_db.clone()),
            user_repository: DynamoDBRepositoryImpl::new(dynamo_db.clone()),
        }
    }
}
