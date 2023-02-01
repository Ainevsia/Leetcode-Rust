pub struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();
        seats.iter().zip(students).fold(0, |acc, (&x, y)| {
            acc + (x-y).abs()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_moves_to_seat(vec![3,1,5], vec![2,7,4]), 4);
        assert_eq!(Solution::min_moves_to_seat(vec![4,1,5,9], vec![1,3,2,6]), 7);
        assert_eq!(Solution::min_moves_to_seat(vec![2,2,6,6], vec![1,3,2,6]), 4);
    }
}
