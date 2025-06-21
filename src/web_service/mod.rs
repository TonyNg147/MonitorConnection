use axum::Router;

pub mod routes;
pub mod views;

use crate::web_service::routes::{register_tracker_venue, statics, venues};
use eyre::{Context, Result};
use tokio::{net::TcpListener, task::JoinSet};

pub async fn start_web_service(js: &mut JoinSet<Result<()>>) -> Result<()> {
    let app = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let routes = Router::new()
        .merge(statics::route())
        .merge(venues::route())
        .merge(register_tracker_venue::route());

    js.spawn(async move {
        axum::serve(app, routes)
            .await
            .wrap_err("Cannot establish the server")?;
        Ok(())
    });

    Ok(())
}
