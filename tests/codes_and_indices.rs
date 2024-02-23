use rstest::rstest;

use crate::common::CTree;

mod common;

#[rstest]
#[case(
    &[CTree { s: "E", i: 0, m: None}],
    "EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM"
)]
#[case(
    &[CTree { s: "E", i: 0, m: None}, CTree { s: "E", i: 44, m: None}],
    "EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccMEBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM"
)]
fn test_deser(#[case] codes: &[CTree], #[case] stream: &str) {
    let actual_codes = common::flatten_codes_recursively(common::parse(stream));
    assert_eq!(
        actual_codes, codes,
        "expected: {codes:?}, actual: {actual_codes:?}"
    );
}
