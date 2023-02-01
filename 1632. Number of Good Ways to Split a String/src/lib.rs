struct UnionFind {
    p: Vec<usize>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            size: vec![1;n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn unite(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {   // not in a same set
            if self.size[pa] > self.size[pb] {  // a bigger
                self.p[pb] = pa;    // merge b into a, a is root
                self.size[pa] += self.size[pb];
            } else {
                self.p[pa] = pb;
                self.size[pb] += self.size[pa];
            }
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        use std::collections::BTreeMap;
        let mut d: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for i in 0..m {
            for j in 0..n {
                d.entry(matrix[i][j]).or_default().push((i, j));
            }
        }
        let mut row_max = vec![0; m];
        let mut col_max = vec![0; n];
        let mut ans = vec![vec![0; n]; m];
        for (_, ps) in d { // point with the same value
            // maybe 1 point
            let mut uf = UnionFind::new(m + n);
            let mut rank = vec![0; m+n];    // max rank
            for &(i, j) in ps.iter() {
                uf.unite(i, j + m);
            }
            // will not change because won't unite any more
            for &(i, j) in ps.iter() {
                rank[uf.find(i)] = *vec![row_max[i], col_max[j], rank[uf.find(i)]]
                    .iter().max().unwrap();
            }
            for &(i, j) in ps.iter() {
                let t = 1 + rank[uf.find(i)];
                row_max[i] = t; // increasing
                col_max[j] = t;
                ans[i][j] = t; // answer
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::matrix_rank_transform(vec![vec![1,2],vec![3,4]]);
        assert_eq!(result, vec![vec![1,2],vec![2,3]]);
    }
}
