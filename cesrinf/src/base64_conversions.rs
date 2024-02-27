//! Base 64 Encoding with URL and Filename Safe Alphabet
//! https://datatracker.ietf.org/doc/html/rfc4648#section-5

use crate::error::Error;

pub trait TryFromBase64<A: Sized> {
    fn try_from_base64(src: A) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait FromBase64<A: Sized> {
    fn from_base64(src: A) -> Self
    where
        Self: Sized;
}

impl TryFromBase64<&str> for usize {
    fn try_from_base64(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Err(Error::EmptyStream);
        }

        let mut count: usize = 0;
        for c in s.chars() {
            count = (count << 6) + usize::try_from_base64(c)?;
        }

        Ok(count)
    }
}

impl TryFromBase64<char> for usize {
    fn try_from_base64(src: char) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match src {
            'A'..='Z' => Ok(src as usize - 'A' as usize),
            'a'..='z' => Ok(src as usize - 'a' as usize + 26),
            '0'..='9' => Ok(src as usize - '0' as usize + 52),
            '-' => Ok(62),
            '_' => Ok(63),
            _ => Err(Error::InvalidCountChar(src)),
        }
    }
}

impl FromBase64<usize> for String {
    fn from_base64(mut src: usize) -> Self
    where
        Self: Sized,
    {
        let mut dst = String::new();
        loop {
            let n = src % 64;
            dst.push(char::try_from_base64(n).expect("valid base64 char"));
            src >>= 6;
            if src == 0 {
                break;
            }
        }
        dst.chars().rev().collect()
    }
}

impl TryFromBase64<usize> for char {
    fn try_from_base64(src: usize) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let dst = match src {
            0..=25 => char::from_u32('A' as u32 + src as u32),
            26..=51 => char::from_u32('a' as u32 + src as u32 - 26),
            52..=61 => char::from_u32('0' as u32 + src as u32 - 52),
            62 => Some('-'),
            63 => Some('_'),
            _ => return Err(Error::InvalidCountNum(src)),
        }
        .ok_or_else(|| Error::InvalidCountNum(src))?;

        Ok(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("A", 0)]
    #[case("B", 1)]
    #[case("Z", 25)]
    #[case("a", 26)]
    #[case("b", 27)]
    #[case("z", 51)]
    #[case("0", 52)]
    #[case("1", 53)]
    #[case("9", 61)]
    #[case("-", 62)]
    #[case("_", 63)]
    #[case("AA", 0)]
    #[case("AB", 1)]
    #[case("BA", 64)]
    #[case("BB", 65)]
    #[case("CB", 129)]
    #[case("ACB", 129)]
    fn test_str_to_usize_valid(#[case] s: &str, #[case] count: usize) {
        assert_eq!(
            usize::try_from_base64(s),
            Ok(count),
            "s: {s}, count: {count}"
        );
    }

    #[rstest]
    #[case("!", '!')]
    #[case("!!", '!')]
    #[case("abc!", '!')]
    fn test_str_to_usize_invalid(#[case] s: &str, #[case] c: char) {
        assert_eq!(
            usize::try_from_base64(s),
            Err(Error::InvalidCountChar(c)),
            "s: {s}, c: {c}"
        );
    }

    #[rstest]
    #[case(0, "A")]
    #[case(1, "B")]
    #[case(25, "Z")]
    #[case(26, "a")]
    #[case(27, "b")]
    #[case(51, "z")]
    #[case(52, "0")]
    #[case(53, "1")]
    #[case(61, "9")]
    #[case(62, "-")]
    #[case(63, "_")]
    #[case(64, "BA")]
    #[case(65, "BB")]
    fn test_usize_to_str(#[case] n: usize, #[case] s: &str) {
        assert_eq!(&String::from_base64(n), s, "n: {n}, s: {s}");
    }
}
