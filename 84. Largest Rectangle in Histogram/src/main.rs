fn main() {
    assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
}

struct Solution {}

impl Solution {
    // i cannot write such brain-consuming code!
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        if heights.len() == 0 { return 0 }
        heights.push(0);
        let mut res = 0;
        let mut buf = vec![0];
        let mut i = 1;
        while i < heights.len() {
            if buf.is_empty() { buf.push(i); i += 1; continue }
            let end = buf.len() - 1;
            if heights[buf[end]] <= heights[i] {
                buf.push(i); i += 1; continue
            }
            let h = heights[buf[end]];
            let w = if end == 0 { i } else { i - buf[end-1] - 1 };
            let rect = h * w as i32;
            res = if rect > res { rect } else { res };
            buf.pop();
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,2]), 3);

    }
}
