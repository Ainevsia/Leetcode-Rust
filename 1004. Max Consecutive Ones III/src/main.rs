fn main() {
    assert_eq!(Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2), 6);
}

struct Solution {}

impl Solution {
    /// this is classic two pointers or substring problem !
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let n = a.len();
        let mut i = 0;
        let mut j = 0;
        // init
        loop {
            if j >= n {
                // already overflow
                return a.len() as i32
            }
            if a[j] == 1 { j += 1 }
            else if a[j] == 0 && k > 0 {
                k -= 1;
                j += 1;
            }
            break;
        }
        // now j points to the right-most zero k can reach
        let mut res = j - i;
        while j < n {
            while a[i] == 1 { i += 1 }
            i += 1;
            j += 1;
            while j < n && a[j] == 1 { j += 1 }
            res = if res < j - i { j - i } else { res };
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2), 6);
        assert_eq!(Solution::longest_ones(vec![1,0,1,0,1,1,1,0,0,1,0,1,0,1,0,1,0,1,1,1,0,0,0,1,0,1,0,1], 2), 7);
        assert_eq!(Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3), 10);
    }
}