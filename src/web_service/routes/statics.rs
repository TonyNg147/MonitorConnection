use axum::{
    extract::Request,
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue,
    },
    middleware::Next,
    response::Response,
    routing::get,
    Router,
};

pub fn route() -> Router {
    Router::new()
        .route(
            "/js/htmx.js",
            get(include_str!("../../../extern/htmx.js"))
                .layer(axum::middleware::from_fn(attach_as_static)),
        )
        .route(
            "/js/htmx-ext-sse.js",
            get(include_str!("../../../extern/htmx-ext-sse.js"))
                .layer(axum::middleware::from_fn(attach_as_static)),
        )
        .route(
            "/css/main.css",
            get(include_str!("../../../css/main.css"))
                .layer(axum::middleware::from_fn(attach_as_css)),
        )
}

async fn attach_as_static(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600, immutable"),
    );
    response
}

async fn attach_as_css(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));
    response
}
