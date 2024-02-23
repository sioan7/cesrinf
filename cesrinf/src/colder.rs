//! https://trustoverip.github.io/tswg-cesr-specification/#table-1
//! https://github.com/WebOfTrust/keripy/blob/development/src/keri/core/parsing.py#L23

use std::fmt::Display;

pub enum ColdCodexStatus {
    Evt,
    Txt,
    Bny,
}

impl TryFrom<ColdCodex> for ColdCodexStatus {
    type Error = String;

    fn try_from(value: ColdCodex) -> Result<Self, Self::Error> {
        match value {
            ColdCodex::Free => Err("cold codex starting with free bits has no status".to_string()),
            ColdCodex::CtB64 | ColdCodex::OpB64 => Ok(ColdCodexStatus::Txt),
            ColdCodex::Json | ColdCodex::MGPK1 | ColdCodex::Cbor | ColdCodex::MGPK2 => {
                Ok(ColdCodexStatus::Evt)
            }
            ColdCodex::CtOpB2 => Ok(ColdCodexStatus::Bny),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ColdCodex {
    /// Not taken
    Free,
    /// Count code base64 (text)
    CtB64,
    /// Op code base64 (text)
    OpB64,
    /// JSON map event start
    Json,
    /// MGPK fixed map event start
    MGPK1,
    /// CBOR map event start
    Cbor,
    /// MGPK Big 16 or 32 map event start
    MGPK2,
    /// Count code or op code base2 (binary)
    CtOpB2,
}

impl From<u8> for ColdCodex {
    fn from(first_stream_byte: u8) -> Self {
        use ColdCodex::*;

        match first_stream_byte >> 5 {
            0b000 => Free,
            0b001 => CtB64,
            0b010 => OpB64,
            0b011 => Json,
            0b100 => MGPK1,
            0b101 => Cbor,
            0b110 => MGPK2,
            0b111 => CtOpB2,
            _ => unreachable!("all 8 possibilities of the 3 bits covered"),
        }
    }
}

impl Display for ColdCodex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ColdCodex::Free => "free",
            ColdCodex::CtB64 => "count code base64 (text)",
            ColdCodex::OpB64 => "op code base64 (text)",
            ColdCodex::Json => "JSON map event start",
            ColdCodex::MGPK1 => "MGPK fixed map event start",
            ColdCodex::Cbor => "CBOR map event start",
            ColdCodex::MGPK2 => "MGPK Big 16 or 32 map event start",
            ColdCodex::CtOpB2 => "count code or op code base2 (binary)",
        };

        write!(f, "{}", s)
    }
}
