use crate::decoder::{is_lowercase_letter, is_uppercase_letter};

#[derive(Debug, PartialEq)]
pub struct Codeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    pub hs: usize,
    pub ss: usize,
    pub fs: usize,
}

impl<'a> Codeage<'a> {
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

pub fn codeage_indexed(s: &str) -> Option<Codeage<'static>> {
    Some(match s {
        "A" => Codeage {
            selector: "A",
            description: "Ed25519 indexed signature both same",
            hs: 1,
            ss: 1,
            fs: 88,
        },
        "B" => Codeage {
            selector: "B",
            description: "Ed25519 indexed signature current only",
            hs: 1,
            ss: 1,
            fs: 88,
        },
        "C" => Codeage {
            selector: "C",
            description: "ECDSA secp256k1 indexed sig both same",
            hs: 1,
            ss: 1,
            fs: 88,
        },
        "D" => Codeage {
            selector: "D",
            description: "ECDSA secp256k1 indexed sig current only",
            hs: 1,
            ss: 1,
            fs: 88,
        },
        "0A" => Codeage {
            selector: "0A",
            description: "Ed448 indexed signature dual",
            hs: 2,
            ss: 2,
            fs: 156,
        },
        "0B" => Codeage {
            selector: "0B",
            description: "Ed448 indexed signature current only",
            hs: 2,
            ss: 2,
            fs: 156,
        },
        // TODO: add all codes
        _ => return None,
    })
}

pub fn codeage(s: &str) -> Option<Codeage<'static>> {
    Some(match s {
        "--" => Codeage {
            selector: "--",
            description: "Universal genus version code",
            hs: 3,
            ss: 5,
            fs: 8,
        },
        "-A" => Codeage {
            selector: "-A",
            description: "Generic pipeline group up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: 4,
        },
        "-0A" => Codeage {
            selector: "-0A",
            description: "Generic pipeline group up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: 8,
        },
        // TODO: add the rest of count codes
        "A" => Codeage {
            selector: "A",
            description: "Seed of Ed25519 private key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "B" => Codeage {
            selector: "B",
            description: "Ed25519 non-transferable prefix public verification key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "C" => Codeage {
            selector: "C",
            description: "X25519 public encryption key, may be converted from Ed25519 public key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "D" => Codeage {
            selector: "D",
            description: "Ed25519 public verification key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "E" => Codeage {
            selector: "E",
            description: "Blake3-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "F" => Codeage {
            description: "F",
            selector: "Blake2b-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "G" => Codeage {
            description: "G",
            selector: "Blake2s-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "H" => Codeage {
            description: "H",
            selector: "SHA3-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "I" => Codeage {
            description: "I",
            selector: "SHA2-256 Digest",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "J" => Codeage {
            description: "J",
            selector: "Seed of ECDSA secp256k1 private key",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "K" => Codeage {
            description: "K",
            selector: "Seed of Ed448 private key",
            hs: 1,
            ss: 0,
            fs: 76,
        },
        "L" => Codeage {
            description: "L",
            selector: "X448 public encryption key",
            hs: 1,
            ss: 0,
            fs: 76,
        },
        "M" => Codeage {
            description: "M",
            selector: "Short number 2-byte b2",
            hs: 1,
            ss: 0,
            fs: 4,
        },
        "N" => Codeage {
            description: "N",
            selector: "Big number 8-byte b2",
            hs: 1,
            ss: 0,
            fs: 12,
        },
        "O" => Codeage {
            description: "O",
            selector: "X25519 private decryption key/seed may be converted from Ed25519 key/seed",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "P" => Codeage {
            description: "P",
            selector: "P",
            hs: 1,
            ss: 0,
            fs: 124,
        },
        "Q" => Codeage {
            description: "Q",
            selector: "Q",
            hs: 1,
            ss: 0,
            fs: 44,
        },
        "0A" => Codeage {
            description: "0A",
            selector: "0A",
            hs: 2,
            ss: 0,
            fs: 24,
        },
        "0B" => Codeage {
            description: "0B",
            selector: "0B",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0C" => Codeage {
            description: "0C",
            selector: "0C",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0D" => Codeage {
            description: "0D",
            selector: "0D",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0E" => Codeage {
            description: "0E",
            selector: "0E",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0F" => Codeage {
            description: "0F",
            selector: "0F",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0G" => Codeage {
            description: "0G",
            selector: "0G",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "0H" => Codeage {
            description: "0H",
            selector: "0H",
            hs: 2,
            ss: 0,
            fs: 8,
        },
        "0I" => Codeage {
            description: "0I",
            selector: "0I",
            hs: 2,
            ss: 0,
            fs: 88,
        },
        "1AAA" => Codeage {
            description: "1AAA",
            selector: "1AAA",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAB" => Codeage {
            description: "1AAB",
            selector: "1AAB",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAC" => Codeage {
            description: "1AAC",
            selector: "1AAC",
            hs: 4,
            ss: 0,
            fs: 80,
        },
        "1AAD" => Codeage {
            description: "1AAD",
            selector: "1AAD",
            hs: 4,
            ss: 0,
            fs: 80,
        },
        "1AAE" => Codeage {
            description: "1AAE",
            selector: "1AAE",
            hs: 4,
            ss: 0,
            fs: 56,
        },
        "1AAF" => Codeage {
            description: "1AAF",
            selector: "1AAF",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "1AAG" => Codeage {
            description: "1AAG",
            selector: "1AAG",
            hs: 4,
            ss: 0,
            fs: 36,
        },
        "1AAH" => Codeage {
            description: "1AAH",
            selector: "1AAH",
            hs: 4,
            ss: 0,
            fs: 100,
        },
        "1AAI" => Codeage {
            description: "1AAI",
            selector: "1AAI",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "1AAJ" => Codeage {
            description: "1AAJ",
            selector: "1AAJ",
            hs: 4,
            ss: 0,
            fs: 48,
        },
        "2AAA" => Codeage {
            description: "2AAA",
            selector: "2AAA",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "3AAA" => Codeage {
            description: "3AAA",
            selector: "3AAA",
            hs: 4,
            ss: 0,
            fs: 8,
        },
        "4A" => Codeage {
            description: "4A",
            selector: "4A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "5A" => Codeage {
            description: "5A",
            selector: "5A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "6A" => Codeage {
            description: "6A",
            selector: "6A",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "7AAA" => Codeage {
            description: "7AAA",
            selector: "7AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "8AAA" => Codeage {
            description: "8AAA",
            selector: "8AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "9AAA" => Codeage {
            description: "9AAA",
            selector: "9AAA",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "4B" => Codeage {
            description: "4B",
            selector: "4B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "5B" => Codeage {
            description: "5B",
            selector: "5B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "6B" => Codeage {
            description: "6B",
            selector: "6B",
            hs: 2,
            ss: 2,
            fs: usize::MAX,
        },
        "7AAB" => Codeage {
            description: "7AAB",
            selector: "7AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "8AAB" => Codeage {
            description: "8AAB",
            selector: "8AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "9AAB" => Codeage {
            description: "9AAB",
            selector: "9AAB",
            hs: 4,
            ss: 4,
            fs: usize::MAX,
        },
        "_" => Codeage {
            selector: "_",
            description: "TBD",
            hs: 0,
            ss: 0,
            fs: 0,
        },
        _ => return None,
    })
}
