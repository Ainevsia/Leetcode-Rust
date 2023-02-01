pub struct Solution {}

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let node_num = k.pow(n as u32 - 1) as usize;
        let edge_num = k.pow(n as u32    ) as usize;
        let mut node = vec![k - 1; node_num];  // the max next of this node
        let mut res = "0".repeat(n as usize - 1).to_string();
        let mut idx = 0;
        for _ in 0..edge_num  {
            dbg!(idx, &node, &res);
            let edge = node[idx];
            node[idx] -= 1;
            res.push_str(&edge.to_string());
            idx = (idx * k as usize + edge as usize) % node_num;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::crack_safe(3, 3), "4");
    }
}
