use horrorshow::html;

pub fn build() -> String {
    html!(
        head {
            script(src="/js/htmx.js") {}
            script(src="/js/htmx-ext-sse.js") {}
            link(rel="stylesheet",href="/css/main.css");
        }
    )
    .to_string()
}
