fn main() {
    Solution::ways(vec![
        "A..".into(),
        "AAA".into(),
        "...".into()], 3);
    let a = vec!["123".to_owned(),"34".to_owned()];
    let b = &a;
    let c = a.iter();
    let d = b.iter();
    println!("finish");

}

trait MyTrait {
    fn iterr(&self) {
        println!("123123");
    }
}

impl MyTrait for Vec<String> {
    // fn iterr(&self) {
        
    //     println!("44444");
    // }
}

struct A {
    num: Vec<Vec<i32>>
}

impl A {
    fn new(pizza: &Vec<String>) -> Self {
        let pizza: Vec<Vec<i32>> = pizza.iter().map(|x: &String| {
            let mut res: Vec<i32> = vec![];
            for i in x.chars() {
                if i == 'A' { res.push(1) }
                else { res.push(0) }
            }
            res
        }).collect::<Vec<Vec<i32>>>();
        let row = pizza.len();
        let col = pizza[0].len();
        let mut num = vec![vec![0; col]; row];
        num[0][0] = pizza[0][0];
        for j in 1..col { num[0][j] = num[0][j-1] + pizza[0][j]; }
        for i in 1..row { num[i][0] = num[i-1][0] + pizza[i][0]; }
        for i in 1..row { for j in 1..col {
            num[i][j] = num[i-1][j] + num[i][j-1] - num[i-1][j-1] + pizza[i][j];
        }}
        // for i in 0..row {
        //     for j in 0..col {
        //         print!("{}", num[i][j]);
        //     }
        // }
        A { num }
    }

    fn has(&self, p1: (usize, usize), p2: (usize, usize)) -> bool {
        // p1 -> inclusive; p2-> exclusive
        let (x1, y1, x2, y2) = (p1.0, p1.1, p2.0 - 1, p2.1 - 1);
        let sum1 = if x1 * y1 == 0 { 0 } else { self.num[x1-1][y1-1] };
        let sum2 = if x1 == 0 { 0 } else { self.num[x1-1][y2] };
        let sum3 = if y1 == 0 { 0 } else { self.num[x2][y1-1] };
        let total = self.num[x2][y2] + sum1 - sum2 - sum3;
        // println!("{}", total);
        total > 0
    }
}

struct Solution {}

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let row = pizza.len();
        let col = pizza[0].len();
        let mut dp = vec![vec![vec![0usize; k as usize + 1]; col]; row];
        dp[0][0][1] = 1;
        let a = A::new(&pizza);
        for z in 2..=k as usize { for i in 0..row { for j in 0..col {
            // now the cake is at (i, j) - inclusive
            if dp[i][j][z-1] == 0 { continue }  // impossible
            // iterate over row
            for x in i + 1..row {   // x -> the below half - inclusive
                if a.has((i,j), (x,col)) && a.has((x,j), (row,col))  {
                    // println!("{:#?}", (i, j, x));
                    dp[x][j][z] += dp[i][j][z-1];
                    dp[x][j][z] %= 1000000007;
                }
            }
            // iterate over col
            for y in j + 1..col {
                if a.has((i,j), (row,y)) && a.has((i, y), (row, col)) {
                    dp[i][y][z] += dp[i][j][z-1];
                    dp[i][y][z] %= 1000000007;
                }
            }
        }}}

        // sum
        let mut ans = 0usize;
        for i in 0..row { for j in 0..col {
            ans += dp[i][j][k as usize];
            ans %= 1000000007;
        }}
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a() {
        let pizza = vec![
            "A..".into(),
            "AAA".into(),
            "...".into()];
        let a = A::new(&pizza);
        let x = a.has((0,0),(1,1));
        assert!(x == true);
    }

    #[test]
    fn basic() {
        assert_eq!(Solution::ways(vec![
            "A..".into(),
            "AAA".into(),
            "...".into()], 3), 3);
    }

    #[test]
    fn basic2() {
        
        assert_eq!(Solution::ways(vec![
            "A..".into(),
            "AA.".into(),
            "...".into()], 3), 1);
    }

    
    #[test]
    fn basic3() {
        assert_eq!(Solution::ways(vec![
            "A..".into(),
            "A..".into(),
            "...".into()], 1), 1);
    }
}