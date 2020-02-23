fn main() {
    assert_eq!(Solution::validate_binary_tree_nodes(4, vec![1,-1,3,-1], vec![2,-1,-1,-1]), true);
}

struct Solution {}

impl Solution {
    /// recursive
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut visited = vec![false; n];
        if Self::visit(&left_child, &right_child, 0, &mut visited) == false { return false }
        visited.iter().all(|x| *x == true)
    }
    
    pub fn visit(l: &Vec<i32>, r: &Vec<i32>, n: usize, v: &mut Vec<bool>) -> bool {
        if v[n] == true { return false }
        v[n] = true;
        let ls = if l[n] != -1 { Self::visit(l, r, l[n] as usize, v) } else { true };
        let rs = if r[n] != -1 { Self::visit(l, r, r[n] as usize, v) } else { true };
        ls && rs
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::validate_binary_tree_nodes(4, vec![1,-1,3,-1], vec![2,-1,-1,-1]), true);
        assert_eq!(Solution::validate_binary_tree_nodes(6, vec![1,-1,-1,4,-1,-1], vec![2,-1,-1,5,-1,-1]), false);

    }
}