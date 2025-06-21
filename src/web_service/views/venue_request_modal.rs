use horrorshow::{html, Raw};

pub fn build() -> String {
    html!(
        script: Raw(r#"
            function openCreateDialog() {
                document.getElementById("createModal").classList.add("active");
                document.getElementById("venue").focus();
            }
        "#);
        div(class="modal", id="createModal", hx-on:htmx:after-request="document.getElementById('createModal').classList.remove('active')") {
            div(class="modal-content") {
                div(class="modal-header") {
                    h2: "Create new Venue";
                    p: "Enter the number of the venue";
                }
                form(hx-post="/register-new-venue", hx-swap="none") {
                    div(class="form-group") {
                        label(for="venue"): "Venue Name";
                        input(type="text", id="venue", placeholder="0..1000", name="venue_id", required) {}
                    }
                    div(class="form-group") {
                        label(for="venue"): Raw("Update time (In Sec)");
                        input(type="text", id="venue", placeholder="Enter the interval the venue will be updated", name="update_time", required) {}
                    }
                    div(class="modal-actions") {
                        button(type="button", class="btn btn-secondary"): "Cancel";
                        button(type="submit", class="btn btn-primary"): "Create Venue";
                    }
                }
            }
        }
    ).to_string()
}
