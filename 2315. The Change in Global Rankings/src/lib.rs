pub fn count_asterisks(s: String) -> i32 {
       s.split('|')
        .enumerate()
        .filter_map(|(x, y)| if x & 1 == 0 { Some(y) } else { None })
        .fold(0, |acc, x| acc + x.chars().filter(|&x| x == '*').count())
    as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = count_asterisks("l|*e*et|c**o|*de|".into());
        assert_eq!(result, 2);
        let result = count_asterisks("iamprogrammer".into());
        assert_eq!(result, 0);
        let result = count_asterisks("yo|uar|e**|b|e***au|tifu|l".into());
        assert_eq!(result, 5);
    }
}
