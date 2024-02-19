use crate::decoder::{is_lowercase_letter, is_uppercase_letter};

#[derive(Debug, PartialEq)]
pub struct IndexerCodeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    pub hs: usize,
    pub ss: usize,
    pub os: usize,
    pub fs: usize,
}

impl<'a> IndexerCodeage<'a> {
    pub fn cs(&self) -> usize {
        self.hs + self.ss
    }

    pub fn vs(&self) -> usize {
        self.fs - self.hs - self.ss
    }

    pub fn ms(&self) -> usize {
        self.ss - self.os
    }

    pub fn ls(&self) -> usize {
        0
    }

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

pub fn codeage(s: &str) -> Option<IndexerCodeage<'static>> {
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
