pub fn get_smallest_string(n: i32, k: i32) -> String {
    let a = (k - n) / 25;   // cnt of ending zero
    let b = (((k - n) % 25) as u8 + 'a' as u8) as char; // if valid, the char not a/z
    std::iter::repeat('a')
        .take(((n - a) as usize).saturating_sub(1))
        .chain(std::iter::once(b).take((n - a) as usize))
        .chain(std::iter::repeat('z').take(a as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_smallest_string(3, 27);
        assert_eq!(result, "aay".to_string());
        let result = get_smallest_string(5, 73);
        assert_eq!(result, "aaszz".to_string());
        let result = get_smallest_string(5, 130);
        assert_eq!(result, "zzzzz".to_string());
    }
}
