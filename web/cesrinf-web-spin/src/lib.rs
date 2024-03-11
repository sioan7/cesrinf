use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn cloud_start(req: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(418)
        .body(Some("hello from spin".into()))?)
}
