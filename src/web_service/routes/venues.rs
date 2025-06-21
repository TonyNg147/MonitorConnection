use std::convert::Infallible;

use axum::{
    response::{
        sse::{Event, KeepAlive},
        Html, Sse,
    },
    routing::get,
    Router,
};
use futures_util::Stream;
use tokio_stream::StreamExt;

use crate::{state::get_state, web_service::views::venue_monitor};

pub fn route() -> Router {
    Router::new()
        .route("/venue_stream", get(sse_handler_for_venue))
        .route("/", get(venue_board))
}

async fn sse_handler_for_venue() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let tx = get_state().tx.subscribe();
    let stream = tokio_stream::wrappers::BroadcastStream::new(tx).filter_map(|msg| match msg {
        Ok(event) => Some(Ok(event)),
        Err(_) => None,
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn venue_board() -> Html<String> {
    Html(venue_monitor::build())
}
