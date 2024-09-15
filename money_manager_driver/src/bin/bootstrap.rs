use std::sync::Arc;

use money_manager_driver::{
    module::Modules,
    startup::{create_app, init_app},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = create_app(Arc::new(modules)).await;
    Ok(())
}
