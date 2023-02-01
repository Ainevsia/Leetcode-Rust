pub struct Solution {}

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeMap;
        let mut buy_orders: BTreeMap<i32, usize> = BTreeMap::new();
        let mut sell_orders: BTreeMap<i32, usize> = BTreeMap::new();
        for order in orders {
            let (price, mut amount, order_type) = (order[0], order[1] as usize, order[2]);
            if order_type == 0 {
                // this is a buy order, search for min sell that key <= price
                while let Some((&key, &value)) = sell_orders.iter().next() {
                    if key <= price { // take this order
                        if amount < value { // consume this buy order
                            *sell_orders.get_mut(&key).unwrap() -= amount;
                            amount = 0;
                            break
                        } else { // consume this sell order
                            sell_orders.remove(&key);
                            amount -= value;
                            if amount == 0 { break }
                        }
                    } else { break }
                }
                if amount > 0 {
                    // insert into buy order
                    *buy_orders.entry(price).or_default() += amount;
                }
            } else {
                while let Some((&key, &value)) = buy_orders.iter().next_back() {
                    if key >= price {
                        if amount < value {
                            *buy_orders.get_mut(&key).unwrap() -= amount;
                            amount = 0;
                            break
                        } else {
                            buy_orders.remove(&key);
                            amount -= value;
                            if amount == 0 { break }
                        }
                    } else { break }
                }
                if amount > 0 {
                    // insert into buy order
                    *sell_orders.entry(price).or_default() += amount;
                }
            }
        }
        let res = buy_orders.iter().fold(0usize, |acc, (_, x)| {
            (acc + x) % 1000000007
        }) + sell_orders.iter().fold(0usize, |acc, (_, x)| {
            (acc + x) % 1000000007
        });
        (res % 1000000007) as i32
    }

    pub fn get_number_of_backlog_orders_2(orders: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut buy_orders: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut sell_orders: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        for order in orders {
            let (price, mut amount, order_type) = (order[0], order[1] as usize, order[2]);
            if order_type == 0 {
                while let Some(&(key, value)) = sell_orders.peek() {
                    if key.0 <= price {
                        if amount < value { // consume this buy order
                            *sell_orders.peek_mut().unwrap() = (key, value - amount);
                            amount = 0;
                            break
                        } else { // consume this sell order
                            sell_orders.pop();
                            amount -= value;
                            if amount == 0 { break }
                        }
                    } else { break }
                }
                if amount > 0 {
                    // insert into buy order
                    buy_orders.push((price, amount));
                }
            } else {
                while let Some(&(key, value)) = buy_orders.peek() {
                    if key >= price {
                        if amount < value {
                            *buy_orders.peek_mut().unwrap() = (key, value - amount);
                            amount = 0;
                            break
                        } else {
                            buy_orders.pop();
                            amount -= value;
                            if amount == 0 { break }
                        }
                    } else { break }
                }
                if amount > 0 {
                    // insert into buy order
                    sell_orders.push((Reverse(price), amount));
                }
            }
        }
        let res = buy_orders.iter().fold(0usize, |acc, (_, x)| {
            (acc + x) % 1000000007
        }) + sell_orders.iter().fold(0usize, |acc, (_, x)| {
            (acc + x) % 1000000007
        });
        (res % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use crate::Solution;

    
    #[test]
    fn basic() {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        map.insert(3, 3);
        map.insert(1, 1);
        map.insert(2, 2);
        let x = map.iter().next();
        dbg!(x);
        let x = map.iter().next_back();
        dbg!(x);

        use std::collections::BinaryHeap;
        let mut map: BinaryHeap<(i32,usize)> = BinaryHeap::new();
        map.push((3,3));
        map.push((1,1));
        map.push((2,2));
        let x = map.peek();
        dbg!(x);

        let mut map = BinaryHeap::new();
        map.push((Reverse(3),3));
        map.push((Reverse(1),1));
        map.push((Reverse(2),2));
        let x = map.peek();
        dbg!(x);
        // unimplemented!()
    }

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_number_of_backlog_orders(
            vec![vec![10,5,0],vec![15,2,1],vec![25,1,1],vec![30,4,0]]
        ), 6);
        assert_eq!(Solution::get_number_of_backlog_orders(
            vec![vec![7,1000000000,1],vec![15,3,0],vec![5,999999995,0],vec![5,1,1]]
        ), 999999984);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::get_number_of_backlog_orders_2(
            vec![vec![10,5,0],vec![15,2,1],vec![25,1,1],vec![30,4,0]]
        ), 6);
        assert_eq!(Solution::get_number_of_backlog_orders_2(
            vec![vec![7,1000000000,1],vec![15,3,0],vec![5,999999995,0],vec![5,1,1]]
        ), 999999984);
    }
}
