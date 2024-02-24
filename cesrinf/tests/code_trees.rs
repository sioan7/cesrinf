use rstest::rstest;

use crate::common::CTree;

mod common;

#[rstest]
#[case(
    &[CTree { s: "--", i: 0, m: None}],
    "--AAABAA"
)]
#[case(
    &[CTree { s: "--", i: 0, m: None}],
    "--AAACAA"
)]
#[case(
    &[CTree { s: "A", i: 0, m: None}],
    "AJ97qKeoQzmWJvqxmeuqIMQbRxHErlNBUsm9BJ2FKX6T"
)]
#[case(
    &[CTree { s: "A", i: 0, m: None}, CTree { s: "A", i: 44, m: None}],
    "AKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgzAKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgz"
)]
#[case(
    &[CTree { s: "B", i: 0, m: None}],
    "BKxy2sgzfplyr-tgwIxS19f2OchFHtLwPWD3v4oYimBx"
)]
#[case(
    &[CTree { s: "D", i: 0, m: None}],
    "DFs8BBx86uytIM0D2BhsE5rrqVIT8ef8mflpNceHo4XH"
)]
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
    &[CTree { s: "F", i: 0, m: None}],
    "FIB_G6cxR8OpbC1js43VpfUU9mKQoUNruYIenypy7_Jj"
)]
#[case(
    &[CTree { s: "G", i: 0, m: None}],
    "GDadJwsilx1z1cK6TpVqnwIRDlaRfhGDPmG3DpczSm0g"
)]
#[case(
    &[CTree { s: "H", i: 0, m: None}],
    "HAHaiEPpdpE6pcFaYtRfHJJnOR3L0Kdq1BGRkEPzdKFj"
)]
#[case(
    &[CTree { s: "I", i: 0, m: None}],
    "IMd157dX7eYwzQqhETvRAmYas4gpylKmQiq3goYvJoZG"
)]
#[case(
    &[CTree { s: "J", i: 0, m: None}],
    "JJ97qKeoQzmWJvqxmeuqIMQbRxHErlNBUsm9BJ2FKX6T"
)]
#[case(
    &[CTree { s: "Q", i: 0, m: None}],
    "QJ97qKeoQzmWJvqxmeuqIMQbRxHErlNBUsm9BJ2FKX6T"
)]
#[case(
    &[CTree { s: "0B", i: 0, m: None}],
    "0BAcbTzvOldV43AGAW6yv1luylr4BPb06B8PiycQ1SbON09QPQW3812nzlzaitf-hmyCSG-mevc3_kOHDT3pj6AA"
)]
#[case(
    &[CTree { s: "0C", i: 0, m: None}],
    "0CBfgMBa5HEyXffL2xvC9BHDBa_0vjt-rD6MFTqfpQo9aXVFkzTIliv-eY3RTpwfbKfIEtZ6bMV0n--NpyWilUfM"
)]
#[case(
    &[CTree { s: "0I", i: 0, m: None}],
    "0ICM-rRAAdKrSrzFlouiZXbNUZ07QMM1IXOaG-gv4TAo4QeQCKZC1z82jJYy_wFkAxgIhbikl3a-nOTXxecF2lEj"
)]
#[case(
    &[CTree { s: "1AAB", i: 0, m: None}],
    "1AABAg299p5IMvuw71HW_TlbzGq5cVOQ7bRbeDuhheF-DPYk"
)]
#[case(
    &[CTree { s: "1AAJ", i: 0, m: None}],
    "1AAJA3cK_P2CDlh-_EMFPvyqTPI1POkw-dr14DANx5JEXDCZ"
)]
#[case(
    &[CTree { s: "1AAG", i: 0, m: None}],
    "1AAG2020-08-22T17c50c09d988921p00c00"
)]
// TODO: add support for digit selectors
// #[case(
//     &[CTree { s: "4", i: 0, m: None}],
//     "4AAGA1s2c1s2c1s4c1s4c1s4a1c1"
// )]
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
#[case(
    &[
        CTree { s: "-F", i: 0, m: Some(vec![
            CTree { s: "E", i: 4, m: None},
        ])},
        CTree { s: "-E", i: 48, m: Some(vec![
            CTree { s: "0A", i: 52, m: None},
        ])},
        CTree { s: "E", i: 76, m: None},
        CTree { s: "-A", i: 120, m: Some(vec![
            CTree { s: "A", i: 124, m: None},
            CTree { s: "A", i: 212, m: None},
            CTree { s: "A", i: 300, m: None},
        ])},
    ],
    "-FAB\
        E_T2_p83_gRSuAYvGhqV3S0JzYEF2dIa-OCPLbIhBO7Y\
        -EAB\
        0AAAAAAAAAAAAAAAAAAAAAAB\
        EwmQtlcszNoEIDfqD-Zih3N6o5B3humRKvBBln2juTEM\
        -AAD\
        AA5267UlFg1jHee4Dauht77SzGl8WUC_0oimYG5If3SdIOSzWM8Qs9SFajAilQcozXJVnbkY5stG_K4NbKdNB4AQ\
        ABBgeqntZW3Gu4HL0h3odYz6LaZ_SMfmITL-Btoq_7OZFe3L16jmOe49Ur108wH7mnBaq2E_0U0N0c5vgrJtDpAQ\
        ACTD7NDX93ZGTkZBBuSeSGsAQ7u0hngpNTZTK_Um7rUZGnLRNJvo5oOnnC1J2iBQHuxoq8PyjdT3BHS2LiPrs2Cg"
)]
fn test_valid_codes(#[case] codes: &[CTree], #[case] stream: &str) {
    let actual_codes = common::flatten_codes_recursively(common::parse(stream));
    assert_eq!(
        actual_codes, codes,
        "expected: {codes:#?}, actual: {actual_codes:#?}"
    );
}
