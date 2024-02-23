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
    "EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM\
        EBdXt3gIXOf2BBWNHdSXCJnFJL5OuQPyM5K0neuniccM"
)]
#[case(
    &[CTree { s: "-A", i: 0, m: Some(vec![
        CTree { s: "A", i: 4, m: None},
        CTree { s: "A", i: 92, m: None},
        CTree { s: "A", i: 180, m: None},
    ])}],
    "-AAD\
        AA5267UlFg1jHee4Dauht77SzGl8WUC_0oimYG5If3SdIOSzWM8Qs9SFajAilQcozXJVnbkY5stG_K4NbKdNB4AQ\
        ABBgeqntZW3Gu4HL0h3odYz6LaZ_SMfmITL-Btoq_7OZFe3L16jmOe49Ur108wH7mnBaq2E_0U0N0c5vgrJtDpAQ\
        ACTD7NDX93ZGTkZBBuSeSGsAQ7u0hngpNTZTK_Um7rUZGnLRNJvo5oOnnC1J2iBQHuxoq8PyjdT3BHS2LiPrs2Cg"
)]
fn test_deser(#[case] codes: &[CTree], #[case] stream: &str) {
    let actual_codes = common::flatten_codes_recursively(common::parse(stream));
    assert_eq!(
        actual_codes, codes,
        "expected: {codes:?}, actual: {actual_codes:?}"
    );
}
