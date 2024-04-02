use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use cesrinf::domain::ParsedData;
use cesrinf::{CesrParser, CesrinfError, EXAMPLES};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct CResponse<T: Serialize, R: Serialize> {
    data: Option<T>,
    err: Option<R>,
}

pub async fn cesr(
    stream: String,
) -> Result<Json<CResponse<ParsedData, CesrinfError>>, (StatusCode, String)> {
    let parser = match CesrParser::new(&stream) {
        Ok(x) => x,
        Err(e) => {
            return Ok(Json(CResponse {
                data: None,
                err: Some(e),
            }))
        }
    };

    let parsed_data = match parser.parse() {
        Ok(x) => x,
        Err(e) => {
            return Ok(Json(CResponse {
                data: None,
                err: Some(e),
            }))
        }
    };

    Ok(Json(CResponse {
        data: Some(parsed_data),
        err: None,
    }))
}
