fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0 }
        let mut sum = 0;
        let mut height = Vec::with_capacity(nums.len() + 1);
        let mut map: Vec<(usize, usize)> = Vec::with_capacity(nums.len() * 2 + 1);
        for _ in 0..map.capacity() {
            map.push((usize::max_value(), usize::max_value()));
        }
        height.push(0);
        let base = nums.len();
        map[base + 0].0 = 0;
        
        // O (n)
        for i in 0..nums.len() {
            sum += if nums[i] == 0 { -1 } else { 1 };
            height.push(sum);
            let idx: usize = (base as i32 + sum) as usize;
            if map[idx].0 == usize::max_value() {
                map[idx].0 = i + 1;
            } else {
                map[idx].1 = i + 1;
            }
        }
        let mut res = 0;

        // O (2n+1)
        for i in 0..map.len() {
            if map[i].1 != usize::max_value() && map[i].1 - map[i].0 > res {
                res = map[i].1 - map[i].0;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_max_length(vec![0,1]), 2);
        assert_eq!(Solution::find_max_length(vec![0,1,1,1,1,0,1,1,1,1]), 2);
    }
}