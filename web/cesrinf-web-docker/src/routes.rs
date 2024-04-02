use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use cesrinf::domain::ParsedData;
use cesrinf::{CesrParser, CesrinfError};
use serde::Serialize;

const EXAMPLES: &[&str] = &[
    "AJ97qKeoQzmWJvqxmeuqIMQbRxHErlNBUsm9BJ2FKX6T",
    "AKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgzAKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgz",
    "0BAcbTzvOldV43AGAW6yv1luylr4BPb06B8PiycQ1SbON09QPQW3812nzlzaitf-hmyCSG-mevc3_kOHDT3pj6AA",
    "1AAJA3cK_P2CDlh-_EMFPvyqTPI1POkw-dr14DANx5JEXDCZ",
    "-AAD\
        AA5267UlFg1jHee4Dauht77SzGl8WUC_0oimYG5If3SdIOSzWM8Qs9SFajAilQcozXJVnbkY5stG_K4NbKdNB4AQ\
        ABBgeqntZW3Gu4HL0h3odYz6LaZ_SMfmITL-Btoq_7OZFe3L16jmOe49Ur108wH7mnBaq2E_0U0N0c5vgrJtDpAQ\
        ACTD7NDX93ZGTkZBBuSeSGsAQ7u0hngpNTZTK_Um7rUZGnLRNJvo5oOnnC1J2iBQHuxoq8PyjdT3BHS2LiPrs2Cg",
    "-FAB\
        E_T2_p83_gRSuAYvGhqV3S0JzYEF2dIa-OCPLbIhBO7Y\
        -EAB\
        0AAAAAAAAAAAAAAAAAAAAAAB\
        EwmQtlcszNoEIDfqD-Zih3N6o5B3humRKvBBln2juTEM\
        -AAD\
        AA5267UlFg1jHee4Dauht77SzGl8WUC_0oimYG5If3SdIOSzWM8Qs9SFajAilQcozXJVnbkY5stG_K4NbKdNB4AQ\
        ABBgeqntZW3Gu4HL0h3odYz6LaZ_SMfmITL-Btoq_7OZFe3L16jmOe49Ur108wH7mnBaq2E_0U0N0c5vgrJtDpAQ\
        ACTD7NDX93ZGTkZBBuSeSGsAQ7u0hngpNTZTK_Um7rUZGnLRNJvo5oOnnC1J2iBQHuxoq8PyjdT3BHS2LiPrs2Cg",
];

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
