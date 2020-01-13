fn main() {
    println!("Hello, world!");
    let r = Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]);
    println!("{:?}", r);
    let a = 3;
    let b = a / 2;
    print!("{}", b);
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let l = l1 + l2;
        if l1 == 0 {
            if l2 % 2 == 0 {  /* divide the middle two */
                return (nums2[l2/2 - 1] + nums2[l2/2]) as f64 / 2.0;
            } else {
                return nums2[(l2 - 1) / 2] as f64;
            }
        } else if l2 == 0 {
            if l1 % 2 == 0 {  /* divide the middle two */
                return (nums1[l1/2 - 1] + nums1[l1/2]) as f64 / 2.0;
            } else {
                return nums1[(l1 - 1) / 2] as f64;
            }
        }

        // main logic
        let mut i = 0;
        let mut j = 0;
        let mut c = 0;
        for _ in 0..l/2 {
            // if l is even, stop at divide left
            // if l is odd,  stop at previous one
            if i == l1 {
                c = nums2[j];
                j += 1;
                continue;
            } else if j == l2 {
                c = nums1[i];
                i += 1;
                continue;
            }
            if nums1[i] < nums2[j] {
                c = nums1[i];
                i += 1;
            } else {
                c = nums2[j];
                j += 1;
            }
            println!("i = {}, j = {}", i, j);
        }
        if l % 2 == 0 {  /* divide c and the next num */
            if i == l1 {
                return (c + nums2[j]) as f64 / 2.0;
            } else if j == l2 {
                return (c + nums1[i]) as f64 / 2.0;
            }
            let n = if nums1[i] < nums2[j] {
                nums1[i]
            } else {
                nums2[j]
            };
            println!("c = {}, n = {}", c, n);
            return (c + n) as f64 / 2.0;
        } else {
            if i == l1 {
                return nums2[j] as f64;
            } else if j == l2 {
                return nums1[i] as f64;
            }
            return if nums1[i] < nums2[j] {
                nums1[i]
            } else {
                nums2[j]
            } as f64;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![0,0], vec![0,0]), 0.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2,3,4]), 2.5);
        
    }
}
