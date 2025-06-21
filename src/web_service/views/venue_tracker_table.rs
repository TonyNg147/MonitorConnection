use horrorshow::html;

pub fn build() -> String {
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
                    tbody(id="venues-table", hx-ext="sse", sse-connect="/venue_stream", sse-swap="new-venue", hx-swap="beforeend") {}
                }
            }
        }
    ).to_string()
}
