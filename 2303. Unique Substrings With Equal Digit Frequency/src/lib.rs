pub fn calculate_tax(brackets: Vec<Vec<i32>>, mut income: i32) -> f64 {
    let mut res = 0f64;
    let mut prevt = 0;
    for i in brackets {
        let t = i[0];
        let p = i[1] as f64 / 100f64;
        if income <= t - prevt {
            res += income as f64 * p;
            break
        } else {
            income -= t - prevt;
            res += (t - prevt) as f64 * p;
            prevt = t;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(calculate_tax(vec![vec![3,50],vec![7,10],vec![12,25]], 10), 2.65000);
    }
}
