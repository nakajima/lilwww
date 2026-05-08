use maud::{DOCTYPE, Markup, html};

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
            }
            body {
                (body)
            }
        }
    }
}
