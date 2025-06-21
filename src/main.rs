use tokio::task::JoinSet;

use crate::web_service::start_web_service;

mod state;
mod web_service;

#[tokio::main]
async fn main() {
    let mut js = JoinSet::new();

    let _res = start_web_service(&mut js).await;

    while let Some(service) = js.join_next().await {
        println!("Result of service is {service:?}");
    }
}
