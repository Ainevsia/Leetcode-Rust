fn main() {
    assert_eq!(Solution::has_valid_path(vec![vec![2,4,3],vec![6,5,2]]), true);
}

struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut uf = WeightedQuickUnionUF::new(m * 2 - 1, n * 2 - 1);
        // initialize union - find complete
        for i in 0..m {
            for j in 0..n {
                let r = grid[i][j];
                if (r == 2 || r == 5 || r == 6) && i > 0 {
                    uf.union((i * 2, j * 2), (i * 2 - 1, j * 2))
                }
                if (r == 1 || r == 3 || r == 5) && j > 0 {
                    uf.union((i * 2, j * 2), (i * 2, j * 2 - 1))
                }
                if (r == 2 || r == 3 || r == 4) && i < m - 1 {
                    uf.union((i * 2, j * 2), (i * 2 + 1, j * 2))
                }
                if (r == 1 || r == 4 || r == 6) && j < n - 1 {
                    uf.union((i * 2, j * 2), (i * 2, j * 2 + 1))
                }
            }
        }
        uf.connected((0, 0), (m * 2 - 2, n * 2 - 2))
    }
}

struct WeightedQuickUnionUF {
    id: Vec<Vec<(usize, usize)>>,
    sz: Vec<Vec<usize>>,
}

impl WeightedQuickUnionUF {
    pub fn new(m: usize, n: usize) -> WeightedQuickUnionUF {
        let mut res = WeightedQuickUnionUF {
            id: vec![vec![(0,0); n]; m],
            sz: vec![vec![1; n]; m],
        };
        for i in 0..m {
            for j in 0..n {
                res.id[i][j] = (i, j);
            }
        }
        res
    }

    pub fn find(&self, mut p: (usize, usize)) -> (usize, usize) {
        while p != self.id[p.0][p.1] {
            p = self.id[p.0][p.1];
        }
        p
    }

    pub fn union(&mut self, p: (usize, usize), q: (usize, usize)) {
        let cp = self.find(p);
        let cq = self.find(q);
        if cp == cq { return }
        if self.sz[cp.0][cp.1] < self.sz[cq.0][cq.1] {
            self.id[cp.0][cp.1] = cq;
            self.sz[cq.0][cq.1] += self.sz[cp.0][cp.1];
        } else {
            self.id[cq.0][cq.1] = cp;
            self.sz[cp.0][cp.1] += self.sz[cq.0][cq.1];
        }
    }

    pub fn connected(&self, p: (usize, usize), q: (usize, usize)) -> bool {
        self.find(p) == self.find(q)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::has_valid_path(vec![vec![2,4,3],vec![6,5,2]]), true);
    }
}