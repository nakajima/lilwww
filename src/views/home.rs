use axum::response::Html;
use maud::{DOCTYPE, html};
use tracing::instrument;

#[instrument]
pub async fn get() -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                title { "Sup" }
            }
            body {
                "Hello world. Hi. Greetings. Salutations."
            }
        }
    };

    Html(markup.into_string())
}
