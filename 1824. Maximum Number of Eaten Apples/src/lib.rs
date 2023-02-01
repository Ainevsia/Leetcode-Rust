pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    let obstacles = obstacles.iter().map(|&x| x as usize).collect::<Vec<_>>();
    let mut v = vec![vec![i32::MAX; 3]; obstacles.len()];
    // x y z
    // a b c
    v[0][0] = 1;
    v[0][1] = 0;
    v[0][2] = 1;
    if obstacles[0] > 0 {
        v[0][obstacles[0] - 1] = i32::MAX;
    }
    for i in 1..obstacles.len() {
        for j in 1..=3 {    // first find the direct jump lane
            if obstacles[i] != j && obstacles[i - 1] != j {
                v[i][j - 1] = v[i - 1][j - 1];
            }
        }
        for j in 1..=3 {
            if obstacles[i] == j { continue }
            else if obstacles[i - 1] == j {
                v[i][j - 1] = *v[i].iter().min().unwrap() + 1;
            }
        }
    }
    
    *v[obstacles.len() - 1].iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(min_side_jumps(vec![0,1,2,3,0]), 2);
        assert_eq!(min_side_jumps(vec![0,1,1,3,3,0]), 0);
        assert_eq!(min_side_jumps(vec![0,2,1,0,3,0]), 2);
    }
}
