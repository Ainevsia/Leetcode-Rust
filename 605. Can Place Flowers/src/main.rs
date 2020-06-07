fn main() {
    assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1), true);
}

struct Solution {}

impl Solution {
    // len in [1, 20000], n is a non-negative integer which won't exceed the input array size.
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        // GREDDY
        
        println!("flowerbed = {:?}", flowerbed);
        if n == 0 { return true }
        if flowerbed.len() == 1 {
            if flowerbed[0] == 1 && n > 0 { return false }
            if flowerbed[0] == 0 {
                if n > 1 { return false }
                else if n == 1 { return true }
            }
        }
        let mut to_plant = n;
        if flowerbed[0] == 0 && to_plant > 0 && flowerbed[1] == 0 {
            flowerbed[0] = 1;
            to_plant -= 1;
        }
        for i in 1..flowerbed.len() - 1 {
            if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 && flowerbed[i] == 0 && to_plant > 0 {
                flowerbed[i] = 1;
                to_plant -= 1;
            }
        }
        let last_idx = flowerbed.len() - 1;
        if flowerbed[last_idx] == 0 && flowerbed[last_idx - 1] == 0 && to_plant > 0 {
            flowerbed[last_idx] = 1;
            to_plant -= 1;
        }
        println!("flowerbed = {:?}", flowerbed);
        to_plant == 0
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::can_place_flowers(vec![1,0], 1), false);
        assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 2), false);
        assert_eq!(Solution::can_place_flowers(
            vec![1,0,0,1,0,0,0,0,1,0,0,0,1,1,0,0,0,1,1,1,1,0,0,1,0,0,1,1,1,1,1,1,1,1,1], 4), false);
    }
}
