use anyhow::Result;
use askama::Template;
use cesrinf::EXAMPLES;
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
};

#[http_component]
fn cesrinf_web_spin(_req: Request) -> Result<impl IntoResponse> {
    let t = IndexTemplate {
        examples: EXAMPLES.iter().map(|x| x.to_string()).collect(),
    }
    .to_string();
    Ok(Response::new(200, t))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    examples: Vec<String>,
}
