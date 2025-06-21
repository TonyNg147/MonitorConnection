use horrorshow::{html, Raw};

use crate::web_service::views::{components::head, venue_request_modal, venue_tracker_table};

pub fn build() -> String {
    html!(
        head {
            : Raw(head::build());
        }
        body {
            div(class="header") {
                h1: "Venue Connection Monitor";
                p: "Real-time monitoring of 1000 venue connection"
            }
            div(class="stats-bar") {
                div(class="stats-container") {
                    div(class="stat") {
                        div(class="stat-number", id="total-venues"): 1000;
                        div(class="stat-label"): "Total Venues"
                    }
                    div(class="stat") {
                        div(class="stat-number", id="connected-count"): 0;
                        div(class="stat-label"): "Connected"
                    }
                }
                div(class="controls") {
                    button(class="btn btn-success"): "Enable All";
                    button(class="btn btn-secondary", onclick="openCreateDialog()"): "Create Venue";
                }
            }
            : Raw(venue_tracker_table::build());
            : Raw(venue_request_modal::build());
        }
    )
    .to_string()
}
