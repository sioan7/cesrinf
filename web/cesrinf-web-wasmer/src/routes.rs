use askama::Template;
use axum::response::IntoResponse;
use cesrinf::EXAMPLES;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    examples: Vec<String>,
}

pub async fn index() -> impl IntoResponse {
    IndexTemplate {
        examples: EXAMPLES.iter().map(|x| x.to_string()).collect(),
    }
}
