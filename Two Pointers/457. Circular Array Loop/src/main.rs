fn main() {
    assert_eq!(Solution::circular_array_loop(vec![2,-1,1,2,2]), true);
    let j = 0usize.wrapping_add(-2i32 as usize) % 5;
    println!("j = {:#?}", j);
    let j = 4usize.wrapping_add(1i32 as usize) % 5;
    println!("j = {:#?}", j);
    println!("var = {:#?}", -1i32 as usize);
    let x = (-1i32 as usize) % 5;
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 { return false }
        let mut vec = vec![-1; nums.len()]; // -1 -> not examined
        for i in 0..nums.len() {
            if vec[i] == 0 { continue } // failed way
            vec[i] = 1;
            let asc = if nums[i] > 0 { true } else { false };
            let mut j = Self::next_idx(i, nums[i], nums.len());
            if i == j { vec[i] = 0; continue }
            println!("i = {:#?}", i);
            loop {
                println!("j = {:#?}", j);
                if vec[j] == 0 || asc && nums[j] < 0 || !asc && nums[j] > 0 {
                    Self::fail(i, j, &nums, &mut vec); break
                } else if vec[j] == 1 {
                    return true
                } else {
                    let nextj = Self::next_idx(j, nums[j], nums.len());
                    if nextj == j {
                        Self::fail(i, j, &nums, &mut vec); break
                    }
                    vec[j] = 1;
                    j = nextj;
                }
            }
        }
        false
    }

    pub fn next_idx(i: usize, num: i32, len: usize) -> usize {
        ((num + len as i32) as usize + i) % len
    }

    /// reach a fail node, fail this way
    pub fn fail(i: usize, j: usize, nums: &Vec<i32>, vec: &mut Vec<i32>) {
        let mut idx = i;
        while idx != j {
            let next = Self::next_idx(idx, nums[idx], nums.len());
            vec[idx] = 0;
            idx = next;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
   fn basic() {
        assert_eq!(Solution::circular_array_loop(vec![2,-1,1,2,2]), true);
        assert_eq!(Solution::circular_array_loop(vec![-1,2]), false);
        assert_eq!(Solution::circular_array_loop(vec![-2,1,-1,-2,-2]), false);
        assert_eq!(Solution::circular_array_loop(vec![-1,-1]), true);
        assert_eq!(Solution::circular_array_loop(vec![-1]), false);
        assert_eq!(Solution::circular_array_loop(vec![4,3,2,1,1,-1]), false);
        
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::circular_array_loop(vec![-1,-2,-3,-4,-5]), false);

    }
}