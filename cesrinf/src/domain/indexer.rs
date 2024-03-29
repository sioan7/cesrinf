use serde::Serialize;

use crate::decoder::{is_lowercase_letter, is_uppercase_letter};

#[derive(Debug, PartialEq, Serialize)]
pub struct IndexerCodeage {
    pub selector: &'static str,
    pub description: &'static str,
    /// hard size in chars (fixed) part of code size
    pub hs: usize,
    /// soft size in chars, (Variable) part of code size
    pub ss: usize,
    /// other size in chars, when soft part includes two Variable values
    pub os: usize,
    /// full size in chars where fs = hs + ss + vs
    pub fs: usize,
}

impl IndexerCodeage {
    /// code size in chars (derived value), where cs = hs + ss
    pub fn cs(&self) -> usize {
        self.hs + self.ss
    }

    /// value size in chars
    pub fn vs(&self) -> usize {
        self.fs - self.hs - self.ss
    }

    /// main size in chars, (derived) when soft part provides two Variable values where ms = ss – os
    pub fn ms(&self) -> usize {
        self.ss - self.os
    }

    /// lead size in bytes to pre-pad raw binary bytes
    pub fn ls(&self) -> usize {
        0
    }

    /// pad size in chars Base64 encoded
    pub fn ps(&self) -> usize {
        let universal_selector = &self.selector[0..1];
        match universal_selector {
            x if is_uppercase_letter(x) || is_lowercase_letter(x) => 2,
            "0" | "3" => 0,
            "2" => 2,
            _ => panic!("unrecognized universal selector `{}`", universal_selector),
        }
    }
}

pub fn codeage(s: &str) -> Option<IndexerCodeage> {
    Some(match s {
        "A" => IndexerCodeage {
            selector: "A",
            description: "Ed25519 indexed signature both same",
            hs: 1,
            ss: 1,
            os: 0,
            fs: 88,
        },
        "B" => IndexerCodeage {
            selector: "B",
            description: "Ed25519 indexed signature current only",
            hs: 1,
            ss: 1,
            os: 0,
            fs: 88,
        },
        "C" => IndexerCodeage {
            selector: "C",
            description: "ECDSA secp256k1 indexed sig both same",
            hs: 1,
            ss: 1,
            os: 0,
            fs: 88,
        },
        "D" => IndexerCodeage {
            selector: "D",
            description: "ECDSA secp256k1 indexed sig current only",
            hs: 1,
            ss: 1,
            os: 0,
            fs: 88,
        },
        "0A" => IndexerCodeage {
            selector: "0A",
            description: "Ed448 indexed signature dual",
            hs: 2,
            ss: 2,
            os: 1,
            fs: 156,
        },
        "0B" => IndexerCodeage {
            selector: "0B",
            description: "Ed448 indexed signature current only",
            hs: 2,
            ss: 2,
            os: 1,
            fs: 156,
        },
        "2A" => IndexerCodeage {
            selector: "2A",
            description: "Ed25519 indexed sig big dual",
            hs: 6,
            ss: 4,
            os: 2,
            fs: 92,
        },
        "2B" => IndexerCodeage {
            selector: "2B",
            description: "Ed25519 indexed sig big current only",
            hs: 6,
            ss: 4,
            os: 2,
            fs: 92,
        },
        "2C" => IndexerCodeage {
            selector: "2C",
            description: "ECDSA secp256k1 indexed sig big dual",
            hs: 6,
            ss: 4,
            os: 2,
            fs: 92,
        },
        "2D" => IndexerCodeage {
            selector: "2D",
            description: "ECDSA secp256k1 idx sig big current only",
            hs: 6,
            ss: 4,
            os: 2,
            fs: 92,
        },
        "3A" => IndexerCodeage {
            selector: "3A",
            description: "Ed448 indexed signature big dual",
            hs: 8,
            ss: 6,
            os: 3,
            fs: 160,
        },
        "3B" => IndexerCodeage {
            selector: "3B",
            description: "Ed448 indexed signature big current only",
            hs: 8,
            ss: 6,
            os: 3,
            fs: 160,
        },
        _ => return None,
    })
}
