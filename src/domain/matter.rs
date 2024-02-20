use crate::decoder::{is_lowercase_letter, is_uppercase_letter};

#[derive(Debug, PartialEq)]
pub struct MatterCodeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    pub hs: usize,
    pub ss: usize,
    pub fs: usize,
}

impl<'a> MatterCodeage<'a> {
    pub fn cs(&self) -> usize {
        self.hs + self.ss
    }

    pub fn vs(&self) -> usize {
        self.fs - self.hs - self.ss
    }

    pub fn ls(&self) -> usize {
        let universal_selector = &self.selector[0..1];
        match universal_selector {
            x if is_uppercase_letter(x) || is_lowercase_letter(x) => 0,
            "0" | "1" | "4" | "7" | "-" => 0,
            "2" | "5" | "8" => 1,
            "3" | "6" | "9" => 2,
            "_" => 0,
            _ => panic!("unrecognized universal selector `{}`", universal_selector),
        }
    }

    pub fn ps(&self) -> usize {
        let universal_selector = &self.selector[0..1];
        match universal_selector {
            x if is_uppercase_letter(x) || is_lowercase_letter(x) => 1,
            "1" | "4" | "7" | "-" => 0,
            "2" | "5" | "8" => 1,
            "0" | "3" | "6" | "9" => 2,
            "_" => 0,
            _ => panic!("unrecognized universal selector `{}`", universal_selector),
        }
    }
}

pub fn codeage(s: &str) -> Option<MatterCodeage<'static>> {
    Some(match s {
        "--" => MatterCodeage {
            selector: "--",
            description: "Universal genus version code",
            hs: 3,
            ss: 5,
            fs: 8,
        },
        "-A" => MatterCodeage {
            selector: "-A",
            description: "Generic pipeline group up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: 4,
        },
        "-0A" => MatterCodeage {
            selector: "-0A",
            description: "Generic pipeline group up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: 8,
        },
        // TODO: add the rest of count codes
        "A" => MatterCodeage {
            selector: "A",
            description: "Seed of Ed25519 private key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "B" => MatterCodeage {
            selector: "B",
            description: "Ed25519 non-transferable prefix public verification key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "C" => MatterCodeage {
            selector: "C",
            description: "X25519 public encryption key, may be converted from Ed25519 public key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "D" => MatterCodeage {
            selector: "D",
            description: "Ed25519 public verification key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "E" => MatterCodeage {
            selector: "E",
            description: "Blake3-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "F" => MatterCodeage {
            description: "F",
            selector: "Blake2b-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "G" => MatterCodeage {
            description: "G",
            selector: "Blake2s-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "H" => MatterCodeage {
            description: "H",
            selector: "SHA3-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "I" => MatterCodeage {
            description: "I",
            selector: "SHA2-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "J" => MatterCodeage {
            description: "J",
            selector: "Seed of ECDSA secp256k1 private key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "K" => MatterCodeage {
            description: "K",
            selector: "Seed of Ed448 private key",
            hs: 1,
            ss: 0,
            fs: 76,
        },
        "L" => MatterCodeage {
            description: "L",
            selector: "X448 public encryption key",
            hs: 1,
            ss: 0,
            fs: 76,
        },
        "M" => MatterCodeage {
            description: "M",
            selector: "Short number 2-byte b2",
            hs: 1,
            ss: 0,
            fs: 4,
        },
        "N" => MatterCodeage {
            description: "N",
            selector: "Big number 8-byte b2",
            hs: 1,
            ss: 0,
            fs: 12,
        },
        "O" => MatterCodeage {
            description: "O",
            selector: "X25519 private decryption key/seed may be converted from Ed25519 key/seed",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        // TODO: update the rest of the selectors
        "P" => MatterCodeage {
            description: "P",
            selector: "P",
            hs: 1,
            ss: 0,
            fs: 124,
        },
        "Q" => MatterCodeage {
            description: "Q",
            selector: "Q",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "0A" => MatterCodeage {
            description: "0A",
            selector: "0A",
            hs: 2,
            ss: 0,
            fs: 24,
        },
        "0B" => MatterCodeage {
            description: "0B",
            selector: "0B",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0C" => MatterCodeage {
            description: "0C",
            selector: "0C",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0D" => MatterCodeage {
            description: "0D",
            selector: "0D",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0E" => MatterCodeage {
            description: "0E",
            selector: "0E",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0F" => MatterCodeage {
            description: "0F",
            selector: "0F",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0G" => MatterCodeage {
            description: "0G",
            selector: "0G",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0H" => MatterCodeage {
            description: "0H",
            selector: "0H",
            hs: 2,
            ss: 0,
            fs: 8,
        },
        "0I" => MatterCodeage {
            description: "0I",
            selector: "0I",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "1AAA" => MatterCodeage {
            description: "1AAA",
            selector: "1AAA",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAB" => MatterCodeage {
            description: "1AAB",
            selector: "1AAB",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAC" => MatterCodeage {
            description: "1AAC",
            selector: "1AAC",
            hs: 4,
            ss: 0,
            fs: 80,
        },
        "1AAD" => MatterCodeage {
            description: "1AAD",
            selector: "1AAD",
            hs: 4,
            ss: 0,
            fs: 80,
        },
        "1AAE" => MatterCodeage {
            description: "1AAE",
            selector: "1AAE",
            hs: 4,
            ss: 0,
            fs: 56,
        },
        "1AAF" => MatterCodeage {
            description: "1AAF",
            selector: "1AAF",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "1AAG" => MatterCodeage {
            description: "1AAG",
            selector: "1AAG",
            hs: 4,
            ss: 0,
            fs: 36,
        },
        "1AAH" => MatterCodeage {
            description: "1AAH",
            selector: "1AAH",
            hs: 4,
            ss: 0,
            fs: 100,
        },
        "1AAI" => MatterCodeage {
            description: "1AAI",
            selector: "1AAI",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAJ" => MatterCodeage {
            description: "1AAJ",
            selector: "1AAJ",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "2AAA" => MatterCodeage {
            description: "2AAA",
            selector: "2AAA",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "3AAA" => MatterCodeage {
            description: "3AAA",
            selector: "3AAA",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "4A" => MatterCodeage {
            description: "4A",
            selector: "4A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "5A" => MatterCodeage {
            description: "5A",
            selector: "5A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "6A" => MatterCodeage {
            description: "6A",
            selector: "6A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "7AAA" => MatterCodeage {
            description: "7AAA",
            selector: "7AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "8AAA" => MatterCodeage {
            description: "8AAA",
            selector: "8AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "9AAA" => MatterCodeage {
            description: "9AAA",
            selector: "9AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "4B" => MatterCodeage {
            description: "4B",
            selector: "4B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "5B" => MatterCodeage {
            description: "5B",
            selector: "5B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "6B" => MatterCodeage {
            description: "6B",
            selector: "6B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "7AAB" => MatterCodeage {
            description: "7AAB",
            selector: "7AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "8AAB" => MatterCodeage {
            description: "8AAB",
            selector: "8AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "9AAB" => MatterCodeage {
            description: "9AAB",
            selector: "9AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "_" => MatterCodeage {
            selector: "_",
            description: "TBD",
            hs: 1,
            ss: 0,
            fs: 1,
        },
        _ => return None,
    })
}
