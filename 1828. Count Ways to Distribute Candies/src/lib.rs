pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    for i in queries {
        let (x, y, r) = (i[0], i[1], i[2]);
        let mut res = 0;
        for j in points.iter() {
            let (u, v) = (j[0], j[1]);
            let d = (u - x).pow(2) + (v - y).pow(2);
            if d <= r.pow(2) {
                res += 1
            }
        }
        ans.push(res);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            count_points(
            vec![vec![1,3],vec![3,3],vec![5,3],vec![2,2]], 
            vec![vec![2,3,1],vec![4,3,1],vec![1,1,2]]
        ), 
            vec![3,2,2]);
        assert_eq!(
            count_points(
            vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]], 
            vec![vec![1,2,2],vec![2,2,2],vec![4,3,2],vec![4,3,3]]
        ),
            vec![2,3,2,4]);

    }
}
