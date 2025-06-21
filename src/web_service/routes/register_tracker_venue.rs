use axum::{http::StatusCode, response::IntoResponse, routing::post, Form, Router};
use serde::Deserialize;

use crate::state::{get_state, seed_venue_with_default_connectivity};

#[derive(Deserialize, Debug)]
struct VenueNumber {
    venue_id: String,
    update_time: usize,
}

async fn register_new_venue(Form(venue_info): Form<VenueNumber>) -> impl IntoResponse {
    println!("Received registered Info {:?}", venue_info);
    match get_state().connections.entry(venue_info.venue_id.clone()) {
        dashmap::Entry::Occupied(_) => {
            (StatusCode::BAD_REQUEST, "Cannot insert already existed one")
        }
        dashmap::Entry::Vacant(vacant_entry) => {
            vacant_entry
                .insert_entry(seed_venue_with_default_connectivity(
                    venue_info.venue_id,
                    venue_info.update_time,
                ))
                .get_mut()
                .run()
                .await;
            (StatusCode::OK, "Register new venue successfully")
        }
    }
    .into_response()
}

pub fn route() -> Router {
    Router::new().route("/register-new-venue", post(register_new_venue))
}
