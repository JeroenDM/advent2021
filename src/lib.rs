use std::fmt;
use std::str;

pub fn str_to_num<T>(s: &str) -> T
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    s.to_string().trim().parse::<T>().unwrap()
}

pub fn str_to_numbers<T>(s: &str, delimiters: &[char]) -> Vec<T>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    s.split(&delimiters[..])
        .filter(|&c| c != "")
        .map(str_to_num::<T>)
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_i32() {
        assert_eq!(str_to_num::<i32>("42"), 42);
    }

    #[test]
    fn parse_to_vec_i32() {
        let s = "1,2,3,4";
        assert_eq!(str_to_numbers::<i32>(s, &[',']), vec![1, 2, 3, 4])
    }

    #[test]
    fn parse_to_vec_i32_multiple_delimiters() {
        let s = "1 2 3\n4 ";
        assert_eq!(str_to_numbers::<i32>(s, &[' ', '\n']), vec![1, 2, 3, 4])
    }
}
