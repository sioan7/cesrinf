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

    pub fn parse(self) -> Result<ParsedData<'a>, Error<'a>> {
        handcrafted_decoder::decode(self.stream)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::matter;

    use super::*;

    #[rstest]
    #[case("E", "EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM")]
    fn test_parsing(#[case] code: &str, #[case] matter: &str) {
        let parser = CesrParser::new(matter).unwrap();
        let pd = parser.parse().unwrap();
        assert_eq!(
            pd.msgs,
            vec![crate::domain::Msg::Matter {
                codeage: matter::codeage(code).unwrap(),
                istart: 0,
                indexed: None,
            }]
        );
        // assert_eq!(codeage(code).unwrap(), pd.codeage);
    }
}
