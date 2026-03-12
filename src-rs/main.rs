use axum::{
    Json, Router,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::WithRejection;
use tokio::net::TcpListener;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;
mod common;
mod dto;
mod services;

use crate::common::Res;

use crate::dto::M3u8MergeRequest;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server_host = std::env::var("SERVER_HOST").map_or("127.0.0.1".to_string(), |v| v);
    let server_port = std::env::var("SERVER_PORT").map_or("3001".to_string(), |v| v);

    let app = Router::new()
        .route("/config", get(get_config_handler))
        .route("/m3u8merge", post(m3u8merge_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = format!("{}:{}", server_host, server_port);

    let listener = TcpListener::bind(&addr).await.expect("failed to bind");

    tracing::debug!("listening on http://{}", &addr);

    axum::serve(listener, app).await.expect("server error");
}

/// 获取全局配置
async fn get_config_handler() -> impl IntoResponse {
    let data = services::config::get_app_config().await;
    Res::success(data)
}

async fn m3u8merge_handler(
    WithRejection(Json(body), _): WithRejection<Json<M3u8MergeRequest>, Res<()>>,
) -> impl IntoResponse {
    let res = services::m3u8merge(body).await;
    if res.is_err() {
        return Res::error(res.err().unwrap());
    }
    Res::success(res.unwrap())
}
