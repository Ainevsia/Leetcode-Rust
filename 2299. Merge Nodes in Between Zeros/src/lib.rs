pub fn strong_password_checker_ii(password: String) -> bool {
    if password.len() < 8 { return false }
    let mut prev = None;
    let mut u = false;
    let mut l = false;
    let mut d = false;
    let mut s = false;
    for c in password.chars() {
        if c.is_uppercase() { u = true }
        if c.is_lowercase() { l = true }
        if c.is_digit(10) { d = true }
        if "!@#$%^&*()-+".contains(c) { s = true }
        if let Some(p) = prev {
            if p == c { return false }
        }
        prev = Some(c);
    }
    u && l && d && s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(strong_password_checker_ii("IloveLe3tcode!".into()), true);
        assert_eq!(strong_password_checker_ii("Me+You--IsMyDream".into()), false);
        assert_eq!(strong_password_checker_ii("1aB!".into()), false);
    }
}
