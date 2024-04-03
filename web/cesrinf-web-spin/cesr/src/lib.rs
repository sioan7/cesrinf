use anyhow::anyhow;
use cesrinf::domain::ParsedData;
use cesrinf::{CesrParser, CesrinfError};
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Serialize)]
pub struct CResponse {
    data: Option<ParsedData>,
    err: Option<CesrinfError>,
}

#[http_component]
fn handle_cesr(req: Request) -> anyhow::Result<impl IntoResponse> {
    let stream = String::from_utf8(req.into_body()).unwrap();
    let parser = match CesrParser::new(&stream) {
        Ok(x) => x,
        Err(e) => {
            let body = match serde_json::to_string(&CResponse {
                data: None,
                err: Some(e),
            }) {
                Ok(x) => x,
                Err(e) => return Err(anyhow!("serialization failed: {}", e)),
            };

            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(body)
                .build());
        }
    };

    let parsed_data = match parser.parse() {
        Ok(x) => x,
        Err(e) => {
            let body = match serde_json::to_string(&CResponse {
                data: None,
                err: Some(e),
            }) {
                Ok(x) => x,
                Err(e) => return Err(anyhow!("serialization failed: {}", e)),
            };

            return Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(body)
                .build());
        }
    };

    let body = match serde_json::to_string(&CResponse {
        data: Some(parsed_data),
        err: None,
    }) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("serialization failed: {}", e)),
    };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
