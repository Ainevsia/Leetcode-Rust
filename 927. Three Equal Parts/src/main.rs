fn main() {
    let a = vec![1,2,3,1,2,3];
    let x = a[0..3] == a[3..];
    println!("x = {:#?}", x);
    assert_eq!(Solution::three_equal_parts(vec![1,0,1,0,1]), [0,3]);
}

struct Solution {}

impl Solution {
    /// intuition: there must be same number of 1 in each part
    /// so there is only one possible solution, no need to simulate
    /// use math to simplify the problem
    /// Tag [Math]
    /// O(n) algorithm
    pub fn three_equal_parts(a: Vec<i32>) -> Vec<i32> {
        // count the number of ones
        let n = a.len();
        let ones = a.iter().filter(|&x| *x == 1).count();
        if ones == 0 { return vec![0, 2] }
        if ones % 3 != 0 { return vec![-1,-1] }
        let ones = ones / 3;
        let (i1, i2, i3, j1, j2, j3);
        let mut i = 0;
        let mut cnt = 0;
        while a[i] == 0 { i += 1 }
        i1 = i; // start of 1 inclusive
        while cnt < ones {
            if a[i] == 1 { cnt += 1 }
            i += 1;
        }
        j1 = i; // end of 1 exclusive
        while a[i] == 0 { i += 1 }
        i2 = i;
        cnt = 0;
        while cnt < ones {
            if a[i] == 1 { cnt += 1 }
            i += 1;
        }
        j2 = i;
        while a[i] == 0 { i += 1 }
        i3 = i;
        cnt = 0;
        while cnt < ones {
            if a[i] == 1 { cnt += 1 }
            i += 1;
        }
        j3 = i;
        let z = n - j3; // number of pending zeros
        if j1+z > i2 || j2+z > i3 { return vec![-1,-1] }
        if a[i1..j1+z] != a[i2..j2+z] || a[i2..j2+z] != a[i3..] { return vec![-1,-1] }
        vec![j1+z-1, j2+z].into_iter().map(|x| x as i32).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::three_equal_parts(vec![1,0,1,0,1]), [0,3]);
        assert_eq!(Solution::three_equal_parts(vec![1,1,0,1,1]), [-1,-1]);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::three_equal_parts(vec![1,1,1,1,1,1]), [1,4]);
        assert_eq!(Solution::three_equal_parts(vec![0,0,0,0,0,0]), [0,2]);
    }
}