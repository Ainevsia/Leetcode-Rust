pub fn greatest_letter(s: String) -> String {
    let mut up = 0usize;
    let mut low = 0usize;
    s.chars().for_each(|x| {
        if x.is_uppercase() {
            up |= 1 << (x as u8 - 'A' as u8);
        } else {
            low |= 1 << (x as u8 - 'a' as u8);
        }
    });
    let i = core::mem::size_of::<usize>() * 8 - (up & low).leading_zeros() as usize;
    if i == 0 { return "".into() }
    ((i as u8 + 'A' as u8 - 1) as char).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greatest_letter("lEeTcOdE".into());
        // assert_eq!(result, 4);
    }
}
