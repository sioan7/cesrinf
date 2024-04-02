pub(crate) mod base64_conversions;
pub(crate) mod colder;
pub(crate) mod computations;
pub(crate) mod decoder;

pub mod domain;
pub mod error;
pub mod parser;

pub use error::Error as CesrinfError;
pub use parser::CesrParser;

pub const EXAMPLES: &[&str] = &[
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
