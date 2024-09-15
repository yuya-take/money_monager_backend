use crate::persistence::dynamodb::DynamoDB;

pub struct RepositoriesModule {}

pub trait RepositoriesModuleExt {}

impl RepositoriesModuleExt for RepositoriesModule {}

impl RepositoriesModule {
    pub fn new(dynamo_db: DynamoDB) -> Self {
        RepositoriesModule {}
    }
}
