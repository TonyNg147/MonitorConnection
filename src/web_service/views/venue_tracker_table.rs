use horrorshow::{html, Raw};

use crate::{
    state::{get_state, VenueDisplayInfo},
    web_service::views::components::venue_info_block,
};

pub fn build() -> String {
    let venues = get_state()
        .connections
        .iter()
        .map(|v| v.value().build_display_venue_info())
        .collect::<Vec<VenueDisplayInfo>>();
    html!(
        div(class="main-content") {
            div(class="table-container") {
                table {
                    thead {
                        tr {
                            th: "Venue Name";
                            th: "Connection Status";
                            th: "Connected Time";
                            th: "Last Update";
                            th: "Actions";
                        }
                    }
                    tbody(id="venues-table", hx-ext="sse", sse-connect="/venue_stream", sse-swap="new-venue", hx-swap="beforeend") {
                        @ for venue in &venues {
                            : Raw(venue_info_block::build(venue, &format!("venue-{}", venue.venue_id)));
                        }
                    }
                }
            }
        }
    ).to_string()
}
