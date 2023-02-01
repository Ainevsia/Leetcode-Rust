pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let n = nums.len(); // 1 <= n <= 10^5
    // ? n = 1 n = 2
    let mut a = vec![vec![0; n]; 4];
    for i in 1..=n-1 {
        // dbg!(nums[i]);
        let b = (i-1) & 1;
        a[b][i] = nums[i-1] + a[b][i-1];
        a[b^1][i] = a[b^1][i-1];
        let i = n - i - 1;
        let b = (i+1) & 1;
        a[b+2][i] = nums[i+1] + a[b+2][i+1];
        a[b^1+2][i] = a[b^1+2][i+1];
    }
    // dbg!(&a);
    let mut res = 0;
    for i in 0..n {
        let b = i & 1;
        if a[b][i] + a[2+b^1][i] == a[b^1][i] + a[2+b][i] {
            res += 1
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ways_to_make_fair(vec![6,1,7,4,1]);
        assert_eq!(result, 0);
        let result = ways_to_make_fair(vec![2,1,6,4]);
        assert_eq!(result, 1);
        let result = ways_to_make_fair(vec![1,1,1]);
        assert_eq!(result, 3);
        let result = ways_to_make_fair(vec![1,2,3]);
        assert_eq!(result, 0);
        let result = ways_to_make_fair(vec![1]);
        assert_eq!(result, 1);
        let result = ways_to_make_fair(vec![1,1]);
        assert_eq!(result, 0);
    }
}
