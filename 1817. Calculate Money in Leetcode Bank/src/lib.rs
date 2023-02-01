pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let mut m: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in logs {
        let (id, time) = (log[0], log[1]);
        m.entry(id).or_default().insert(time);
    }
    let mut res = vec![0; k as usize];
    for (_, s) in m {
        let uam = s.len();
        res[uam - 1] += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(finding_users_active_minutes(
            vec![vec![0,5],vec![1,2],vec![0,2],vec![0,5],vec![1,3]],
             5), vec![0,2,0,0,0]);
        assert_eq!(finding_users_active_minutes(
            vec![vec![1,1],vec![2,2],vec![2,3]],
            4), vec![1,1,0,0]);
    }
}
