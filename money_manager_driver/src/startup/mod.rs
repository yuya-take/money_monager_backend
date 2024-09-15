use std::{env, sync::Arc};

use axum::{
    extract::{Extension, MatchedPath, Request},
    Router,
};
use dotenv::dotenv;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::module::Modules;

pub async fn create_app(modules: Arc<Modules>) {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_error_handling=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = create_router(modules);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch"))
}

pub fn create_router(modues: Arc<Modules>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);
    // let hc_router = Router::new()
    //     .route("/", get(hc_hello))
    //     .route("/dynamodb", get(hc_dynamodb));
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .nest("/hc", hc_router)
        .layer(Extension(modues))
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(|req: &Request| {
            let method = req.method();
            let uri = req.uri();

            let matched_path = req
                .extensions()
                .get::<MatchedPath>()
                .map(|matched_path| matched_path.as_str());
            tracing::info_span!("request", %method, %uri, matched_path)
        }))
}

pub fn init_app() {
    dotenv().ok();
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // ヘルスチェック
        // crate::route::health_check::hc_hello,
        // crate::route::health_check::hc_mysql,
    ),
    components(
        schemas(
        )
    ),
    tags(
        (name = "Health Check", description = "ヘルスチェック"),
        (name = "Book", description = "書籍"),
        (name = "Author", description = "著者")
    ),
)]
pub struct ApiDoc;