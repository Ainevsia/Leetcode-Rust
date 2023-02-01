pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
    if batch_size == 1 { return groups.len() as i32 }
    let a: Vec<i32> = groups.iter().map(|&x| x % batch_size).collect();
    let mut b: Vec<i32> = a.iter().filter(|x| **x > 0).copied().collect();
    let res = (a.len() - b.len()) as i32;
    if b.len() <= 1 { return res + b.len() as i32}

    fn simulate_anneal(w: &mut Vec<i32>, bs: i32) -> i32 {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();
        w.shuffle(&mut rng);
        let mut res = 0;
        let mut t = 1e6;
        while t > 1e-5 {
            let i = rng.gen_range(0, w.len());
            let j = rng.gen_range(0, w.len());
            fn cal(w: &Vec<i32>, b: i32) -> i32 {
                let mut res = 0;
                let mut s = 0;
                for i in 0..w.len() {
                    s += w[i];
                    if s % b == 0 {
                        s = 0;
                        res += 1;
                    }
                }
                if s > 0 { res += 1 }
                res
            }
            let a = cal(w, bs);
            if a > res { res = a }
            w.swap(i, j);
            let b = cal(w, bs);
            if b > res { res = b }
            let delta = b - a;
            if delta < 0 {
                let tr = (delta as f64 / t).exp();
                use rand::distributions::{Distribution, Uniform};
                let step = Uniform::new(0.0, 1.0);
                let mut rng = rand::thread_rng();
                let choice = step.sample(&mut rng);
                if tr < choice {
                    w.swap(i, j);
                }
            }
            t *= 0.97
        }
        res
    }

    let mut add = 0;
    for _ in 0..30 {
        let x = simulate_anneal(&mut b, batch_size);
        if x > add { add = x }
    }
    res + add
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_happy_groups(3, vec![1,2,3,4,5,6]), 4);
        assert_eq!(max_happy_groups(4, vec![1,3,2,5,2,2,1,6]), 4);
    }
}
