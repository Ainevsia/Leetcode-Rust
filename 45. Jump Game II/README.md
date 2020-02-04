## insight 1

the min_jump array will never decrease

## insight 2

the max_dis array will never decrease

## first thought

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut vec = vec![0];
        for idx in 1..nums.len() {
            let mut jmp = i32::max_value();
            println!("idx = {:#?}", idx);
            for i in 0..idx {
                println!("i = {:#?}", i);
                if i + nums[i] as usize >= idx {
                    if vec[i] + 1 < jmp {
                        jmp = vec[i] + 1;
                    }
                }
            }
            vec.push(jmp);
        }
        vec[vec.len() - 1]
    }
}
```

O(n^2) find the shortest jump till n.

## recursive 

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        Self::jump_cnt(&nums)
    }

    pub fn jump_cnt(nums: &[i32]) -> i32 {
        if nums.len() == 1 { return 0 }
        for i in 0..nums.len() {
            if i + nums[i] as usize >= nums.len() - 1 {
                return Self::jump_cnt(&nums[0..i+1]) + 1;
            }
        }
        unimplemented!()
    }
}
```

Same O(n^2), find the most right element to reach the tail element.

## 3rd version

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return 0 }
        let mut cnt = vec![i32::max_value(); nums.len()];
        cnt[0] = 0;
        for i in 0..nums.len() {
            if i + nums[i] as usize >= nums.len() - 1 {
                return cnt[i] + 1;
            }
            for idx in i+1..=i+nums[i] as usize {
                if cnt[i] + 1 < cnt[idx] {
                    cnt[idx] = cnt[i] + 1;
                }
            }
        }
        unimplemented!()
    }
}
```

O(k*n) where k = max(list) also very bad.

# Linear Solution

```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.iter().map(|x| *x as usize).collect();
        let mut position = 0;
        let mut cur_ladder = 0;
        let mut next_ladder = nums[0];
        let mut jump_cnt = 0;
        while position < nums.len() - 1 {
            if position + nums[position] > next_ladder {
                next_ladder = position + nums[position];
            }
            if position == cur_ladder {
                jump_cnt += 1;
                cur_ladder = next_ladder;
                if cur_ladder >= nums.len() - 1 {
                    return jump_cnt
                }
            }
            position += 1;
        }
        0
    }
}
```


Switching [Ladder](https://www.youtube.com/watch?v=vBdo7wtwlXs)