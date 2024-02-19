use crate::{
    colder::ColdCodex,
    decoder::{handcrafted_decoder, ParsedData},
};

/// Parser for a CESR stream.
/// Assume a framed stream.
pub struct Parser<'a> {
    stream: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(stream: &'a str) -> Result<Self, String> {
        // TODO: validation
        // what would the minimum size for the stream be?
        // ensure at first at least one byte
        if stream.is_empty() {
            return Err("empty stream".to_string());
        }

        let first_byte = stream
            .bytes()
            .next()
            .expect("dat contains at least one byte");
        let cold_codex = ColdCodex::from(first_byte);
        if let ColdCodex::Free = cold_codex {
            return Err("cannot parse a stream with a free cold start byte".to_string());
        }

        if let ColdCodex::JSON
        | ColdCodex::CBOR
        | ColdCodex::MGPK1
        | ColdCodex::MGPK2
        | ColdCodex::CtOpB2 = cold_codex
        {
            return Err(format!(
                "this parser only supports text for now but the cold codex was `{}`",
                cold_codex
            ));
        }

        Ok(Self { stream })
    }

    // pub fn pad_size(&self) -> usize {
    //     (3 - (self.stream.len() % 3)) % 3
    // }

    // pub fn minimum_code_size_scaling_factor(&self) -> usize {
    //     match self.pad_size() {
    //         0 => 1,
    //         _ => 0,
    //     }
    // }

    pub fn parse(self) -> Result<ParsedData<'a>, String> {
        handcrafted_decoder::decode(self.stream)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{decoder::Msg, tables::matter};

    use super::*;

    #[rstest]
    #[case("E", "EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM")]
    fn test_parsing(#[case] code: &str, #[case] matter: &str) {
        let parser = Parser::new(matter).unwrap();
        let pd = parser.parse().unwrap();
        assert_eq!(
            pd.matteri,
            vec![Msg::Matter {
                codeage: matter::codeage(code).unwrap(),
            }]
        );
        // assert_eq!(codeage(code).unwrap(), pd.codeage);
    }
}
