fn main() {
    assert_eq!(Solution::get_permutation(3, 3), "213");
}

struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let (mut n, mut k) = (n as usize, k as usize - 1);
        let mut x: Vec<char> = (b'1'..).take(n).map(char::from).collect();
        let mut buf = vec![];
        loop {
            let zone = Self::factorial(n - 1);
            let choice = k / zone;
            buf.push(x.remove(choice));
            k -= choice * zone;
            n -= 1;
            if n == 0 { break buf }
        } .iter().collect()
    }

    /// used for small num a: a!
    pub fn factorial(n: usize) -> usize {
        match n {
            0 | 1 => 1,
            _ => n * Self::factorial(n - 1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }
}