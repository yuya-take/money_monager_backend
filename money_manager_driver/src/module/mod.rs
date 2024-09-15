use money_manager_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::dynamodb::DynamoDB,
};

pub struct Modules {}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;
}

impl Modules {
    pub async fn new() -> Modules {
        let dynamo_db = DynamoDB::new().await;
        Self {}
    }
}
