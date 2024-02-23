use crate::decoder::{is_lowercase_letter, is_uppercase_letter};

#[derive(Debug, PartialEq)]
pub struct MatterCodeage<'a> {
    pub selector: &'a str,
    pub description: &'a str,
    /// hard size in chars (fixed) part of code size
    pub hs: usize,
    /// soft size in chars, (Variable) part of code size
    pub ss: usize,
    /// full size in chars where fs = hs + ss + vs
    pub fs: Option<usize>,
}

impl<'a> MatterCodeage<'a> {
    /// code size in chars (derived value), where cs = hs + ss
    pub fn cs(&self) -> usize {
        self.hs + self.ss
    }

    /// value size in chars
    pub fn vs(&self) -> Option<usize> {
        Some(self.fs? - self.hs - self.ss)
    }

    /// lead size in bytes to pre-pad raw binary bytes
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

    /// pad size in chars Base64 encoded
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
            fs: Some(8),
        },
        "-A" => MatterCodeage {
            selector: "-A",
            description: "Generic pipeline group up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0A" => MatterCodeage {
            selector: "-0A",
            description: "Generic pipeline group up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-B" => MatterCodeage {
            selector: "-B",
            description: "Message + attachments group up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0B" => MatterCodeage {
            selector: "-0B",
            description: "Message + attachments group up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-C" => MatterCodeage {
            selector: "-C",
            description: "Attachments only group up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0C" => MatterCodeage {
            selector: "-0C",
            description: "Attachments only group up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-D" => MatterCodeage {
            selector: "-D",
            description: "Datagram Stream Segment up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0D" => MatterCodeage {
            selector: "-0D",
            description: "Datagram Stream Segment up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-E" => MatterCodeage {
            selector: "-E",
            description: "ESSR wrapper signable up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0E" => MatterCodeage {
            selector: "-0E",
            description: "ESSR wrapper signable up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-F" => MatterCodeage {
            selector: "-F",
            description: "CESR native message top-level fixed field signable up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0F" => MatterCodeage {
            selector: "-0F",
            description: "CESR native message top-level fixed field signable up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        "-G" => MatterCodeage {
            selector: "-G",
            description: "CESR native message top-level field map signable up to 4,095 quadlets/triplets",
            hs: 2,
            ss: 2,
            fs: Some(4),
        },
        "-0G" => MatterCodeage {
            selector: "-0G",
            description: "CESR native message top-level field map signable up to 1,073,741,823 quadlets/triplets",
            hs: 3,
            ss: 5,
            fs: Some(8),
        },
        // TODO: add the rest of count codes
        "A" => MatterCodeage {
            selector: "A",
            description: "Seed of Ed25519 private key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "B" => MatterCodeage {
            selector: "B",
            description: "Ed25519 non-transferable prefix public verification key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "C" => MatterCodeage {
            selector: "C",
            description: "X25519 public encryption key, may be converted from Ed25519 public key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "D" => MatterCodeage {
            selector: "D",
            description: "Ed25519 public verification key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "E" => MatterCodeage {
            selector: "E",
            description: "Blake3-256 Digest",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "F" => MatterCodeage {
            selector: "F",
            description: "Blake2b-256 Digest",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "G" => MatterCodeage {
            selector: "G",
            description: "Blake2s-256 Digest",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "H" => MatterCodeage {
            selector: "H",
            description: "SHA3-256 Digest",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "I" => MatterCodeage {
            selector: "I",
            description: "SHA2-256 Digest",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "J" => MatterCodeage {
            selector: "J",
            description: "Seed of ECDSA secp256k1 private key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "K" => MatterCodeage {
            selector: "K",
            description: "Seed of Ed448 private key",
            hs: 1,
            ss: 0,
            fs: Some(76),
        },
        "L" => MatterCodeage {
            selector: "L",
            description: "X448 public encryption key",
            hs: 1,
            ss: 0,
            fs: Some(76),
        },
        "M" => MatterCodeage {
            selector: "M",
            description: "Short number 2-byte b2",
            hs: 1,
            ss: 0,
            fs: Some(4),
        },
        "N" => MatterCodeage {
            selector: "N",
            description: "Big number 8-byte b2",
            hs: 1,
            ss: 0,
            fs: Some(12),
        },
        "O" => MatterCodeage {
            selector: "O",
            description:
                "X25519 private decryption key/seed may be converted from Ed25519 key/seed",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "P" => MatterCodeage {
            selector: "P",
            description: "X25519 124 char qb64 Cipher of 44 char qb64 Seed",
            hs: 1,
            ss: 0,
            fs: Some(124),
        },
        "Q" => MatterCodeage {
            selector: "Q",
            description: "ECDSA secp256r1 256-bit random Seed for private key",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        "R" => MatterCodeage {
            selector: "R",
            description: "Tall 5-byte b2 number",
            hs: 1,
            ss: 0,
            fs: Some(8),
        },
        "S" => MatterCodeage {
            selector: "S",
            description: "Large 11-byte b2 number",
            hs: 1,
            ss: 0,
            fs: Some(16),
        },
        "T" => MatterCodeage {
            selector: "T",
            description: "Great 14-byte b2 number",
            hs: 1,
            ss: 0,
            fs: Some(20),
        },
        "U" => MatterCodeage {
            selector: "U",
            description: "Vast 17-byte b2 number",
            hs: 1,
            ss: 0,
            fs: Some(24),
        },
        "V" => MatterCodeage {
            selector: "V",
            description: "Label1 as one char (bytes) field map label lead size 1",
            hs: 1,
            ss: 0,
            fs: Some(4),
        },
        "W" => MatterCodeage {
            selector: "W",
            description: "Label2 as two char (bytes) field map label lead size 0",
            hs: 1,
            ss: 0,
            fs: Some(4),
        },
        "X" => MatterCodeage {
            selector: "X",
            description: "Tag3 3 B64 encoded chars for field tag or packet type, semver, trait like ‘DND’",
            hs: 1,
            ss: 0,
            fs: Some(4),
        },
        "Y" => MatterCodeage {
            selector: "Y",
            description: "Tag7 7 B64 encoded chars for field tag or packet kind and version KERIVVV",
            hs: 1,
            ss: 0,
            fs: Some(8),
        },
        "Z" => MatterCodeage {
            selector: "Z",
            description: "Blinding factor 256 bits, Cryptographic strength deterministically generated from random salt",
            hs: 1,
            ss: 0,
            fs: Some(44),
        },
        // TODO: complete next codes
        "0A" => MatterCodeage {
            selector: "0A",
            description: "Random salt, seed, nonce, private key, or sequence number of length 128 bits",
            hs: 2,
            ss: 0,
            fs: Some(24),
        },
        "0B" => MatterCodeage {
            selector: "Ed25519 signature",
            description: "0B",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0C" => MatterCodeage {
            selector: "ECDSA secp256k1 signature",
            description: "0C",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0D" => MatterCodeage {
            selector: "Blake3-512 Digest",
            description: "0D",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0E" => MatterCodeage {
            selector: "0E",
            description: "Blake2b-512 Digest",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0F" => MatterCodeage {
            selector: "0F",
            description: "SHA3-512 Digest",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0G" => MatterCodeage {
            selector: "0G",
            description: "SHA2-512 Digest",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0H" => MatterCodeage {
            selector: "0H",
            description: "Long number 4-byte b2",
            hs: 2,
            ss: 0,
            fs: Some(8),
        },
        "0I" => MatterCodeage {
            selector: "0I",
            description: "ECDSA secp256r1 signature",
            hs: 2,
            ss: 0,
            fs: Some(88),
        },
        "0J" => MatterCodeage {
            selector: "0J",
            description: "Tag1 1 B64 encoded char with pre pad for field tag",
            hs: 1, // TODO: ??? check this value, it doesn't seem right; shouldn't it be 2
            ss: 0,
            fs: Some(4),
        },
        "0K" => MatterCodeage {
            selector: "0K",
            description: "Tag2 2 B64 encoded chars for field tag or version VV or trait like ‘EO’",
            hs: 2,
            ss: 0,
            fs: Some(4),
        },
        "0L" => MatterCodeage {
            selector: "0L",
            description: "Tag5 5 B64 encoded chars with pre pad for field tag",
            hs: 2,
            ss: 0,
            fs: Some(8),
        },
        "0M" => MatterCodeage {
            selector: "0M",
            description: "Tag6 6 B64 encoded chars for field tag or protocol kind version like KERIVV (KERI 1.1) or KKKVVV",
            hs: 2,
            ss: 0,
            fs: Some(8),
        },
        "1AAA" => MatterCodeage {
            selector: "1AAA",
            description: "1AAA",
            hs: 4,
            ss: 0,
            fs: Some(48),
        },
        "1AAB" => MatterCodeage {
            selector: "1AAB",
            description: "1AAB",
            hs: 4,
            ss: 0,
            fs: Some(48),
        },
        "1AAC" => MatterCodeage {
            selector: "1AAC",
            description: "1AAC",
            hs: 4,
            ss: 0,
            fs: Some(80),
        },
        "1AAD" => MatterCodeage {
            selector: "1AAD",
            description: "1AAD",
            hs: 4,
            ss: 0,
            fs: Some(80),
        },
        "1AAE" => MatterCodeage {
            selector: "1AAE",
            description: "1AAE",
            hs: 4,
            ss: 0,
            fs: Some(56),
        },
        "1AAF" => MatterCodeage {
            selector: "1AAF",
            description: "1AAF",
            hs: 4,
            ss: 0,
            fs: Some(8),
        },
        "1AAG" => MatterCodeage {
            selector: "1AAG",
            description: "1AAG",
            hs: 4,
            ss: 0,
            fs: Some(36),
        },
        "1AAH" => MatterCodeage {
            selector: "1AAH",
            description: "1AAH",
            hs: 4,
            ss: 0,
            fs: Some(100),
        },
        "1AAI" => MatterCodeage {
            selector: "1AAI",
            description: "1AAI",
            hs: 4,
            ss: 0,
            fs: Some(48),
        },
        "1AAJ" => MatterCodeage {
            selector: "1AAJ",
            description: "1AAJ",
            hs: 4,
            ss: 0,
            fs: Some(48),
        },
        "2AAA" => MatterCodeage {
            selector: "2AAA",
            description: "2AAA",
            hs: 4,
            ss: 0,
            fs: Some(8),
        },
        "3AAA" => MatterCodeage {
            selector: "3AAA",
            description: "3AAA",
            hs: 4,
            ss: 0,
            fs: Some(8),
        },
        "4A" => MatterCodeage {
            selector: "4A",
            description: "4A",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "5A" => MatterCodeage {
            selector: "5A",
            description: "5A",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "6A" => MatterCodeage {
            selector: "6A",
            description: "6A",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "7AAA" => MatterCodeage {
            selector: "7AAA",
            description: "7AAA",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "8AAA" => MatterCodeage {
            selector: "8AAA",
            description: "8AAA",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "9AAA" => MatterCodeage {
            selector: "9AAA",
            description: "9AAA",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "4B" => MatterCodeage {
            selector: "4B",
            description: "4B",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "5B" => MatterCodeage {
            selector: "5B",
            description: "5B",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "6B" => MatterCodeage {
            selector: "6B",
            description: "6B",
            hs: 2,
            ss: 2,
            fs: None,
        },
        "7AAB" => MatterCodeage {
            selector: "7AAB",
            description: "7AAB",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "8AAB" => MatterCodeage {
            selector: "8AAB",
            description: "8AAB",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "9AAB" => MatterCodeage {
            selector: "9AAB",
            description: "9AAB",
            hs: 4,
            ss: 4,
            fs: None,
        },
        "_" => MatterCodeage {
            selector: "_",
            description: "TBD",
            hs: 1,
            ss: 0,
            fs: Some(1),
        },
        _ => return None,
    })
}
