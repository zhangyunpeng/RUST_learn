use crate::global::response::Response;
use crate::handles::user;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn create_router() -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .route("/success_demo", post(success_demo))
        .route("/fail_demo", post(fail_demo))
        .route("/user/info", get(user::user_info::user_info))
        .layer(trace_layer)
}

async fn success_demo() -> Json<Response<u32>> {
    Json(Response::success(0, None))
}

async fn fail_demo() -> Json<Response<u32>> {
    Json(Response::fail(1, "demo error".to_string()))
}
