use crate::{colder::ColdCodex, decoder::handcrafted_decoder, domain::ParsedData, error::Error};

/// Parser for a CESR stream.
/// Assume a framed stream.
pub struct CesrParser<'a> {
    stream: &'a str,
}

impl<'a> CesrParser<'a> {
    pub fn new(stream: &'a str) -> Result<Self, Error> {
        // TODO: validation
        let first_byte = stream.bytes().next().ok_or_else(|| Error::EmptyStream)?;
        let cold_codex = ColdCodex::from(first_byte);
        if let ColdCodex::Free = cold_codex {
            return Err(Error::UnsupportedCodeCodex(
                "cannot parse a stream with a free cold start byte".to_string(),
            ));
        }

        if let ColdCodex::Json
        | ColdCodex::Cbor
        | ColdCodex::MGPK1
        | ColdCodex::MGPK2
        | ColdCodex::CtOpB2 = cold_codex
        {
            return Err(Error::UnsupportedCodeCodex(format!(
                "this parser only supports text for now but the cold codex was `{}`",
                cold_codex
            )));
        }

        Ok(Self { stream })
    }

    pub fn parse(self) -> Result<ParsedData, Error> {
        handcrafted_decoder::decode(self.stream)
    }
}
