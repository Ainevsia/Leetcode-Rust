fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
		let a = a.chars().collect::<Vec<char>>();
		let b = b.chars().collect::<Vec<char>>();
		let mut res = vec!['0'; usize::max(a.len(), b.len()) + 1];
		let mut carry = false;
		for i in (0..res.len()).rev() {
			let offset = res.len() - i;
			let p = if a.len() < offset { None } else { Some(a[a.len() - offset]) };
			let q = if b.len() < offset { None } else { Some(b[b.len() - offset]) };
			match (p, q) {
				(Some(x), Some(y)) => {
					match (x, y) {
						('0', '1') | ('1', '0') => {
							if !carry { res[i] = '1'; carry = false }
						}
						('0', '0') => {
							if carry { res[i] = '1'; carry = false }
						}
						('1', '1') => {
							if carry { res[i] = '1' }
							carry = true
						}
						_ => unreachable!()
					}
				}
				(Some(x), None) | (None, Some(x)) => {
					if carry && x == '0' { res[i] = '1'; carry = false }
					else if !carry { res[i] = x }
				}
				_ => { if carry { res[i] = '1'; carry = false } }
			}

		}
		if res[0] == '0' { res.remove(0); }
		res.iter().collect()
    }
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn basic() {
		assert_eq!(Solution::add_binary(String::from("11"), String::from("1")), "100");
	}
}
