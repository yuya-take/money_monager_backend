use std::env;
use std::sync::Arc;

use aws_config::load_from_env;
use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::Client;
use dotenv::dotenv;

#[derive(Clone)]
pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

impl DynamoDB {
    // DynamoDBインスタンスを作成
    pub async fn new() -> DynamoDB {
        let client = init_client().await;
        DynamoDB {
            client: Arc::new(client),
        }
    }
}

// クライアントの初期化
async fn init_client() -> Client {
    let config = load_from_env().await;
    dotenv().ok();
    let endpoint_url =
        env::var("DYNAMODB_ENDPOINT_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    #[cfg(debug_assertions)]
    let dynamodb_config = Builder::from(&config).endpoint_url(endpoint_url).build();
    #[cfg(not(debug_assertions))]
    let dynamodb_config = Builder::from(&config).build();

    let dynamodb = Client::from_conf(dynamodb_config);

    tracing::info!("DynamoDB client initialized");

    dynamodb
}
