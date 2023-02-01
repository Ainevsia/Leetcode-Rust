pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    // let n = nums.len();
    let revs = nums
        .iter()
        .map(|&num| {
        num
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }).collect::<Vec<_>>();
    let diff = nums
        .iter()
        .enumerate()
        .map(|(idx, &x)| x - revs[idx])
        .collect::<Vec<_>>();
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in diff {
        *map.entry(i).or_default() += 1;
    }
    let mut res: usize = 0;
    for (_, cnt) in map {
        res += cnt * (cnt - 1) / 2;
        res %= 1000000007;
    }
    // for i in 0..n {
    //     for j in i+1..n {
    //         if nums[i] + revs[j] == nums[j] + revs[i] {
    //             res += 1;
    //             res %= 1000000007;
    //         }
    //     }
    // }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        count_nice_pairs(vec![13,10,35,24,76]);
        // unimplemented!()
    }

    #[test]
    fn it_works() {
        assert_eq!(count_nice_pairs(vec![42,11,1,97]), 2);
        assert_eq!(count_nice_pairs(vec![13,10,35,24,76]), 4);
    }
}
