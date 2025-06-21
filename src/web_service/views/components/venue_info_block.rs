use horrorshow::html;

use crate::state::VenueDisplayInfo;

pub fn build(info: VenueDisplayInfo, event_listener: &str) -> String {
    let mut current_status = info.connected_status.chars();
    let capitalized_status = if let Some(first_char) = current_status.next() {
        first_char
            .to_ascii_uppercase()
            .to_string()
            .chars()
            .chain(current_status)
            .collect::<String>()
    } else {
        current_status.collect::<String>()
    };
    html! {
            tr(sse-swap=format!("{}", event_listener),hx-swap="outerHTML") {
                td: format!("{}", info.venue_id);
                td {
                    span(class=format!("status status-{}", info.connected_status)) {
                        span(class="status-dot") {}
                        : &capitalized_status;
                    }
                }
                td: format!("{}", info.initial_time);
                td: format!("{}", info.update_time);
            }
    }
    .to_string()
}
