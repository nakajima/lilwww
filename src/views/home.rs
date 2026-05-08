use crate::{error::AppResult, views::layout};
use axum::response::Html;
use maud::html;
use tracing::instrument;

#[instrument]
pub async fn get() -> AppResult<Html<String>> {
    let body = html! {
        "Hello world. Hi. Greetings. Salutations."
    };

    Ok(Html(layout::page("Sup", body).into_string()))
}
