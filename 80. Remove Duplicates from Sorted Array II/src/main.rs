fn main() {
    let mut v = vec![1,1,1,2,2,3];
    let x = Solution::remove_duplicates(&mut v);
    println!("v = {:#?}", v);
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..nums.len() {
            if cnt < 2 || nums[cnt - 2] < nums[i] {
                nums[cnt] = nums[i];
                cnt += 1;
            }
        }
        cnt as i32
    }


    pub fn remove_duplicates_twopointer(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums.len() as i32 }
        let mut cnt = 1;
        let mut cur = 1;
        let mut dup = false;
        while cur < nums.len() {
            if nums[cur] == nums[cnt - 1] {
                if !dup {
                    nums[cnt] = nums[cur];
                    cnt += 1;
                    dup = true;
                } /* else dup, do nothing */ 
            } else /* not equal */ {
                dup = false;
                nums[cnt] = nums[cur];
                cnt += 1;
            }
            cur += 1;
        }
        cnt as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,1,2,2,3]), 5);
    }
}