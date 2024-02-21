use crate::error::Error;

pub trait TryFromBase62<A: Sized> {
    fn try_from_base62(src: A) -> Result<Self, Error<'static>>
    where
        Self: Sized;
}

pub trait FromBase62<A: Sized> {
    fn from_base62(src: A) -> Self
    where
        Self: Sized;
}

impl TryFromBase62<&str> for usize {
    fn try_from_base62(s: &str) -> Result<Self, Error<'static>> {
        if s.is_empty() {
            return Err(Error::EmptyStream);
        }

        let mut count: usize = 0;
        let mut coeff = s.len() - 1;
        for c in s.chars() {
            count += (62i32.pow(coeff as u32) as usize) * usize::try_from_base62(c)?;
            coeff = coeff.saturating_sub(1);
        }

        Ok(count)
    }
}

impl TryFromBase62<char> for usize {
    fn try_from_base62(src: char) -> Result<Self, Error<'static>>
    where
        Self: Sized,
    {
        match src {
            'A'..='Z' => Ok(src as usize - 'A' as usize),
            'a'..='z' => Ok(src as usize - 'a' as usize + 26),
            '0'..='9' => Ok(src as usize - '0' as usize + 52),
            _ => Err(Error::InvalidCountChar(src)),
        }
    }
}

impl FromBase62<usize> for String {
    fn from_base62(mut src: usize) -> Self
    where
        Self: Sized,
    {
        let mut dst = String::new();
        loop {
            let n = src % 62;
            dst.push(char::try_from_base62(n).expect("valid base62 char"));
            src /= 62;
            if src == 0 {
                break;
            }
        }
        dst.chars().rev().collect()
    }
}

impl TryFromBase62<usize> for char {
    fn try_from_base62(src: usize) -> Result<Self, Error<'static>>
    where
        Self: Sized,
    {
        let dst = match src {
            0..=25 => char::from_u32('A' as u32 + src as u32),
            26..=51 => char::from_u32('a' as u32 + src as u32 - 26),
            52..=61 => char::from_u32('0' as u32 + src as u32 - 52),
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
    #[case("AA", 0)]
    #[case("AB", 1)]
    #[case("BA", 62)]
    #[case("BB", 63)]
    #[case("CB", 125)]
    #[case("ACB", 125)]
    fn test_str_to_usize_valid(#[case] s: &str, #[case] count: usize) {
        assert_eq!(
            usize::try_from_base62(s),
            Ok(count),
            "s: {s}, count: {count}"
        );
    }

    #[rstest]
    #[case("-", '-')]
    #[case("--", '-')]
    #[case("abc-", '-')]
    #[case("_", '_')]
    #[case("__", '_')]
    #[case("abc_", '_')]
    fn test_str_to_usize_invalid(#[case] s: &str, #[case] c: char) {
        assert_eq!(
            usize::try_from_base62(s),
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
    #[case(62, "BA")]
    #[case(62, "BA")]
    #[case(63, "BB")]
    fn test_usize_to_str(#[case] n: usize, #[case] s: &str) {
        assert_eq!(&String::from_base62(n), s, "n: {n}, s: {s}");
    }
}
