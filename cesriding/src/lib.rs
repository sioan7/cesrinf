use cesride::{matter, Diger, Matter};

pub struct CodeMaterial {
    pub code: &'static str,
    pub material: String,
}

pub fn basic_one_character_codes() -> Vec<CodeMaterial> {
    // C
    // let raw = "0123456789abcdef00001111222233334444555566667777888899990000aaaa".as_bytes();
    // let code = matter::Codex::X25519;
    // let verfer = Verfer::new(Some(code), Some(raw), None, None, None).unwrap();
    // println!("{}", verfer.qb64().unwrap());
    vec![
        CodeMaterial {
            code: "F",
            material: Diger::new_with_ser(b"1234567890", Some(matter::Codex::Blake2b_256))
                .unwrap()
                .qb64()
                .unwrap(),
        },
        CodeMaterial {
            code: "G",
            material: Diger::new_with_ser(b"1234567890", Some(matter::Codex::Blake2s_256))
                .unwrap()
                .qb64()
                .unwrap(),
        },
        CodeMaterial {
            code: "F",
            material: Diger::new_with_ser(b"1234567890", Some(matter::Codex::SHA3_256))
                .unwrap()
                .qb64()
                .unwrap(),
        },
        CodeMaterial {
            code: "F",
            material: Diger::new_with_ser(b"1234567890", Some(matter::Codex::SHA2_256))
                .unwrap()
                .qb64()
                .unwrap(),
        },
    ]
}
