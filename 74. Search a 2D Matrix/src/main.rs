fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let array = matrix.into_iter().flatten().collect::<Vec<i32>>();
        if array.len() == 0 { return false }
        let (mut start, mut end) = (0, array.len() - 1);
        while start <= end {
            let mid = (start + end) / 2;
            if target < array[mid] {
                if mid == 0 { return false }    // if remove this line, there will be an underflow panic
                end = mid - 1   // usize should not be -1
            }
            else if target > array[mid] { start = mid + 1 }
            else { return true }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::search_matrix(vec![
            vec![1,   3,  5,  7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 50]
        ], 3), true);
        assert_eq!(Solution::search_matrix(vec![
            vec![1,   3,  5,  7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 50]
        ], 13), false);
        assert_eq!(Solution::search_matrix(vec![
            vec![1,   3,  5,  7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 100]
        ], 100), true);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::search_matrix(vec![
            vec![1, 1],
        ], 0), false);
    }
}