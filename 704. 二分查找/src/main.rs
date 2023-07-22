
fn main() {
    let a = Solution::str_str("hello".into(), "ll".into());
    dbg!(a);
}

struct Solution {}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut v = vec![];
        for s in tokens {
            let a = vec!["+","-","*","/"];
            if a.contains(&&s[..]) {
                let x = v.pop().unwrap();
                let y = v.pop().unwrap();
                match &s[..] {
                    "+" => {v.push(x+y)},
                    "-" => {v.push(y-x)},
                    "*" => {v.push(y*x)},
                    "/" => {v.push(y/x)},
                    _ => {},
                }
            } else {
                v.push(s.parse::<i32>().unwrap());
            }
        }
        v.pop().unwrap()
    }
}
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v = vec![];
        for c in s.chars() {
            if let Some(&x) = v.last() {
                if x == c { v.pop(); }
                else { v.push(c) }
            } else { v.push(c) }
        }
        v.iter().collect()
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![]; // only }])
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                v.push(Self::mutate(c));
            } else {
                if let Some(&x) = v.last() {
                    if x == c { v.pop(); }
                    else { return false }
                } else {
                    return false
                }
            }
        }
        v.is_empty()
    }
    fn mutate(x: char) -> char {
        if x == '(' { return ')' }
        else if x == '[' { return ']' }
        else {return '}'}
    }
}

struct MyStack {
    q: std::collections::VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Self { q: std::collections::VecDeque::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        let n = self.q.len();
        for _ in 1..n {
            let x = self.q.pop_front().unwrap();
            self.q.push_back(x);
        }
        self.q.pop_front().unwrap()
    }
    
    fn top(&mut self) -> i32 {
        let n = self.q.len();
        for _ in 1..n {
            let x = self.q.pop_front().unwrap();
            self.q.push_back(x);
        }
        let x = self.q.pop_front().unwrap();
        self.q.push_back(x);
        return x;
    }
    
    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

struct MyQueue {
    is: Vec<i32>,
    os: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self { is: vec![], os: vec![] }
    }

    fn o2i(&mut self) {
        while let Some(i) = self.os.pop() {
            self.is.push(i);
        }
    }

    fn i2o(&mut self) {
        while let Some(i) = self.is.pop() {
            self.os.push(i);
        }
    }
    
    fn push(&mut self, x: i32) {
        self.o2i();
        self.is.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.i2o();
        self.os.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self.i2o();
        self.os.last().copied().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.is.is_empty() && self.os.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let mut next = vec![0; n];
        let s = s.chars().collect::<Vec<char>>();
        let mut j = 0;
        for i in 1..n {
            while j >= 1 && s[i] != s[j] {
                j = next[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            next[i] = j;    // next 表示以i结尾的字符串最大前后缀长度
        }
        let a = *next.last().unwrap();
        if a == 0 { return false }
        let b = n - a;
        if n % b != 0 { return false }
        else { true }
    }
}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n = needle.len();
        let mut next = vec![0; n];
        let hay = haystack.chars().collect::<Vec<char>>();
        let s = needle.chars().collect::<Vec<char>>();
        let mut j = 0;
        for i in 1..n {
            while j >= 1 && s[i] != s[j] {
                j = next[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            next[i] = j;    // next 表示以i结尾的字符串最大前后缀长度
        }
        // dbg!(&next);
        // build next ok
        if n == 0 { return 0 }
        j = 0;
        for i in 0..hay.len() {
            // dbg!(i, j);
            while j >= 1 && hay[i] != s[j] {
                j = next[j - 1];
            }
            if hay[i] == s[j] {
                j += 1;
            }
            if j == n {
                return (i + 1 - n) as i32
            }
        }
        -1
    }
}

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut v: Vec<char> = s.chars().collect();
        Self::reverse2(&mut v[..n as usize]);
        Self::reverse2(&mut v[n as usize..]);
        Self::reverse2(&mut v[..]);
        v.iter().collect()
    }
    
    pub fn reverse2(s: &mut [char]) {
        let n = s.len();
        for i in 0..n/2 {
            let tmp = s[i];
            s[i] = s[n-1-i];
            s[n-1-i] = tmp;
        }
    }
}

impl Solution {
    // O(n) space O(n) time
    pub fn reverse_words(s: String) -> String {
        let mut v: Vec<char> = vec![];
        for c in s.chars().rev() {
            if c == ' ' {
                let a = v.last();
                if a.is_none() { continue }
                if *a.unwrap() == ' ' { continue }
            }
            v.push(c);
        }
        if v[v.len() - 1] == ' ' { v.pop(); }
        // reverse each word
        let mut start = 0;
        while start < v.len() {
            let mut end = start + 1;
            while end < v.len() && v[end] != ' ' { end += 1 }
            Self::reverse(&mut v[start..end]);
            start = end + 1;
        }
        v.iter().collect()
    }

    pub fn reverse(s: &mut [char]) {
        let n = s.len();
        for i in 0..n/2 {
            let tmp = s[i];
            s[i] = s[n-1-i];
            s[n-1-i] = tmp;
        }
    }
}

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let cnt = s.iter().filter(|&&x| x == ' ').count();
        let n = s.len();
        s.resize(n + 2 * cnt, ' ');
        let mut tail = s.len() - 1;
        let mut head = n as isize - 1;
        while head >= 0 {
            if s[head as usize] == ' ' {
                s[tail] = '0';
                s[tail - 1] = '2';
                s[tail - 2] = '%';
                tail -= 3;
            } else {
                s[tail] = s[head as usize];
                tail -= 1;
            }
            head -= 1;
        }
        s.iter().collect()
    }
}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut v: Vec<char> = vec![];
        let mut rev = true;
        for i in s.chunks(k as usize) {
            if rev {
                v.extend(i.iter().rev());
            } else {
                v.extend(i);
            }
            rev = !rev;
        }
        v.iter().collect()
    }
}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n/2 {
            let tmp = s[i];
            s[i] = s[n-1-i];
            s[n-1-i] = tmp;
        }
    }
}

impl Solution {
    pub fn four_sum(mut v: Vec<i32>, t: i32) -> Vec<Vec<i32>> {
        v.sort();
        let n = v.len();
        let mut res = vec![];
        if n < 4 { return res }
        for i in 0..n-3 {
            if i > 0 && v[i] == v[i-1] { continue }
            for l in (i+3..n).rev() {
                if l < n - 1 && v[l] == v[l+1] { continue }
                let mut j = i + 1;
                let mut k = l - 1;
                while j < k {
                    if v[i] as isize + v[j] as isize + v[k] as isize + v[l] as isize == t as isize {
                        res.push(vec![v[i], v[j], v[k], v[l]]);
                        j += 1;
                        while j < k && v[j] == v[j-1] { j += 1 }
                        k -= 1;
                        while j < k && v[k] == v[k+1] { k -= 1 }
                    } else if (v[i] as isize + v[j] as isize + v[k] as isize + v[l] as isize) < t as isize {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        res
    }
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in magazine.chars() {
            *map.entry(c).or_default() += 1;
        }
        for c in ransom_note.chars() {
            if ! map.contains_key(&c) { return false }
            let a = map.get_mut(&c).unwrap();
            *a -= 1;
            if *a == 0 { map.remove(&c); }
        }
        true
    }
}

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let n = nums1.len();
        use std::collections::HashMap;
        let mut res = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                let target = nums1[i] + nums2[j];
                *map.entry(target).or_default() += 1;
            }
        }
        for i in 0..n {
            for j in 0..n {
                let target = - nums3[i] - nums4[j];
                if map.contains_key(&target) {
                    res += map.get(&target).unwrap();
                }
            }
        }
        res as i32
    }
}


impl Solution {
    pub fn four_sum_count2(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let n = nums1.len();
        use std::collections::HashMap;
        let mut res = 0;
        let mut map3: HashMap<i32, usize> = HashMap::new();
        let mut map4: HashMap<i32, usize> = HashMap::new();
        for i in 0..n {
            *map3.entry(nums3[i]).or_default() += 1;
            *map4.entry(nums4[i]).or_default() += 1;
        }
        for i in 0..n {
            for j in 0..n {
                let mut target = - nums1[i] - nums2[j];
                for (&a, &b) in map3.iter() {
                    let d = target - a;
                    if map4.contains_key(&d) {
                        res += b * map4.get(&d).unwrap();
                    }
                }
            }
        }
        res as i32
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, i) in nums.iter().enumerate() {
            if map.contains_key(&(target - i)) {
                return vec![*map.get(&(target - i)).unwrap(), idx as i32]
            } else {
                map.insert(*i, idx as i32);
            }
        }
        todo!()
    }
}


impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(n);
        fn f(mut n: i32) -> i32 {
            let mut res = 0;
            while n != 0 {
                let x = n % 10;
                res += x * x;
                n /= 10;
            }
            res
        }
        loop {
            n = f(n);
            if n == 1 { return true }
            if set.contains(&n) { return false }
            set.insert(n);
        }
    }
}


impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for i in nums1 {
            set1.insert(i);
        }
        for i in nums2 {
            set2.insert(i);
        }
        set1.intersection(&set2).copied().collect()
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() { return false }
        let mut freq: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_default() += 1;
        }
        for c in t.chars() {
            let entry = freq.entry(c).or_default();
            if *entry == 0 { return false }
            *entry -= 1;
        }
        true
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let dummy = ListNode { val: -1, next: head };
            let mut ptr = &dummy;
            for _ in 0..n { ptr = ptr.next.as_deref()? }
            let mut ptr2 = &dummy as *const ListNode as *mut ListNode;
            while ptr.next.is_some() {
                ptr = ptr.next.as_deref()?;
                ptr2 = (*ptr2).next.as_deref()? as *const ListNode as *mut ListNode;
            }
            let mut rigoff = (*ptr2).next.take()?;
            let nxt = rigoff.next.take();
            (*ptr2).next = nxt;
            dummy.next
        }
    }
}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut cdummy = &mut dummy;
        while let Some(mut x) = head.take() {
            if x.next.is_none() {
                cdummy.next = Some(x);
                return dummy.next;
            }
            if let Some(mut y) = x.next.take() {
                head = y.next.take();
                y.next = Some(x);
                cdummy.next = Some(y);
                cdummy = cdummy.next.as_mut().unwrap();
                cdummy = cdummy.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        while let Some(mut node) = head {
            let tmp = dummy.next.take();
            let n = node.next.take();
            node.next = tmp;
            dummy.next = Some(node);
            head = n;
        }
        dummy.next
    }
}

struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        Self { val: 0, next: None }
    }
    
    fn get(&self, index: i32) -> i32 {
        let mut dummpy = self.next.as_deref();
        let mut cnt = 0;
        while let Some(node) = dummpy {
            if index == cnt { return node.val }
            dummpy = node.next.as_deref();
            cnt += 1;
        }
        -1
    }
    
    fn add_at_head(&mut self, val: i32) {
        self.next = Some(Box::new(
            MyLinkedList {
                val: val, 
                next: self.next.take()
            }
        ));
    }
    
    fn add_at_tail(&mut self, val: i32) {
        let mut dummpy = self;
        while dummpy.next.is_some() {
            dummpy = dummpy.next.as_mut().unwrap();
        }
        dummpy.next = Some(Box::new(
            MyLinkedList {
                val: val, 
                next: None
            }
        ));
    }
    
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut cnt = 0;
        let mut dummpy = self;
        while dummpy.next.is_some() {
            if cnt == index {
                let nxt = dummpy.next.take();
                dummpy.next = Some(Box::new(
                    MyLinkedList {
                        val: val, 
                        next: nxt
                    }
                ));
                return;
            }
            dummpy = dummpy.next.as_mut().unwrap();
            cnt += 1;
        }
        if cnt == index {
            dummpy.next = Some(Box::new(
                MyLinkedList {
                    val: val, 
                    next: None
                }
            ));
        }
    }
    
    fn delete_at_index(&mut self, index: i32) {
        let mut cnt = 0;
        let mut dummpy = self;
        while dummpy.next.is_some() {
            if cnt == index {
                let nxt = dummpy.next.take().unwrap();
                dummpy.next = nxt.next;
                return;
            }
            dummpy = dummpy.next.as_mut().unwrap();
            cnt += 1;
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */


impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = dummy.as_mut();
        while let Some(node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next;
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

impl Solution {
    pub fn remove_elements2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut a = &mut head;
        while a.as_deref_mut().unwrap().next.is_some() {
            let v = a.as_deref_mut().unwrap().next.as_deref().unwrap().val;
            if v == val {
                let mut b = a.as_deref_mut().unwrap().next.take();
                let c = b.as_deref_mut().unwrap().next.take();
                a.as_deref_mut().unwrap().next = c;
            } else {
                let b = &mut a.as_deref_mut().unwrap().next;
                a = b;
            }
        }
        head.unwrap().next
    }
}

impl Solution {
    /// 
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BTreeMap;
        let n = nums.len();
        nums.sort();
        let mut freq = BTreeMap::new();
        let mut i = 0;
        while i < n {
            let c = nums[i];
            let mut j = i + 1;
            while j < n && c == nums[j] { j += 1}
            let cnt = j - i;
            freq.insert(c, cnt);
            i = j;
        }
        let mut v = vec![];
        for i in freq.iter() {
            v.push((*i.0, *i.1));
        }
        dbg!(&v);
        let mut i = 0;
        let mut res = 0;
        while i < v.len() {
            let mut cres = v[i].1;
            let mut j = i + 1;
            while j < v.len() && v[j].0 - v[i].0 <= 2*k {
                cres += v[j].1;
                j += 1;
            }
            res = res.max(cres);
            let mut cres = v[i].1;
            let mut u = i as isize - 1;
            while u >= 0 && v[i].0 - v[u as usize].0 <= 2*k {
                cres += v[u as usize].1;
                u -= 1;
            }
            res = res.max(cres);
            i += 1;
        }
        res as i32
    }
}

impl Solution {
    /// 一维 dp dp[i] 表示以第 i 个字符结尾的最长合法子字符串的长度
    /// 如果不禁止，将 dp[i] 设置为 dp[i-1]+1
    /// 如果禁止，将 dp[i] 设置为 最大的不禁止字符串长度 （优化）
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        todo!()
    }
}

impl Solution {
    /// 在构建freq的同时记录当前的支配元素？需要orderedmap
    /// 用一个变量来记录就行了吧？
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        use std::collections::hash_map::HashMap;
        let mut freq: HashMap<i32, usize> = HashMap::new();
        let mut dominate = vec![0; nums.len()];
        let mut max_cnt = 0;
        let mut max_num = 0;
        // 正向
        for (idx, &i) in nums.iter().enumerate() {
            let cnt = freq.entry(i).or_default();
            *cnt += 1;
            if *cnt > max_cnt {
                max_cnt = *cnt;
                max_num = i;
            }
            let m = idx + 1;
            if max_cnt * 2 > m {
                dominate[idx] = max_num;
            }
        }
        let dom = max_num;
        // 反向
        let mut freq: HashMap<i32, usize> = HashMap::new();
        let mut dominate2 = vec![0; nums.len()];
        let mut max_cnt = 0;
        let mut max_num = 0;
        for (idx, &i) in nums.iter().rev().enumerate() {
            let cnt = freq.entry(i).or_default();
            *cnt += 1;
            if *cnt > max_cnt {
                max_cnt = *cnt;
                max_num = i;
            }
            let m = idx + 1;
            if max_cnt * 2 > m {
                dominate2[n-1-idx] = max_num;
            }
        }
        
        dbg!(&dominate);
        dbg!(&dominate2);
        dbg!(dom);
        for i in 0..n-1 {
            if dominate[i] == dominate2[i+1] && dominate[i] == dom {
                return i as i32;
            }
        }
        -1
    }
}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for (idx, &e) in nums.iter().enumerate() {
            let i = idx + 1;
            if n % i == 0 {
                res += e * e;
            }
        }
        res
    }
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut v = vec![vec![0; n]; n];
        // idx,idx 左上角的坐标， n 这一行的所有元素个数-1  右上角坐标idx,idx+n
        pub fn f(v: &mut Vec<Vec<i32>>, idx: usize, n: usize, start: i32) -> i32 {
            if n == 0 { v[idx][idx] = start; return start + 1 }
            let mut cur = start;
            for j in 0..n {
                v[idx][idx+j] = cur ; cur += 1;
            }
            for i in 0..n {
                v[idx+i][idx+n] = cur ; cur += 1;
            }
            for j in 0..n {
                v[idx+n][idx+n-j] = cur ; cur += 1;
            }
            for i in 0..n {
                v[idx+n-i][idx] = cur ; cur += 1;
            }
            cur
        }
        let mut start = 1;
        let mut x = n as isize - 1;
        let mut i = 0;
        while x >= 0 {
            start = f(&mut v, i, x as usize, start);
            i += 1;
            x -= 2;
        }
        v
    }
}


impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = i32::MAX;
        let mut s = 0;
        let mut a = 0;
        for b in 0..n {
            s += nums[b];
            while s >= target {
                res = res.min((b - a + 1) as i32);
                s -= nums[a];
                a += 1;
            }
        }
        if res == i32::MAX {0} else {res as i32}
    }
}

impl Solution {
    pub fn min_sub_array_len2(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut a = 0;
        let mut b = 0;
        let mut s = 0;
        let mut res = 0;
        while s < target && b < n {
            s += nums[b];b += 1;
        }
        if s < target &&  b == n { return 0 }
        res = b;
        loop {

            while s >= target && a < b {
                s -= nums[a];
                res = res.min(b - a);
                a += 1;
            }
            while s < target && b < n {
                s += nums[b];b += 1;
            }
            if s >= target {
                res = res.min(b - a);
            }
            if b == n && s < target { break }
        }

        res as i32
    }
}


impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v = Vec::with_capacity(n);
        let mut a = 0;
        let mut b = n as isize - 1;
        while a <= b {
            if nums[a as usize].abs() < nums[b as usize].abs() {
                v.push(nums[b as usize]);
                b -= 1;
            } else {
                v.push(nums[a as usize]);
                a += 1;
            }
        }
        v.iter().map(|x| x**x).rev().collect()
    }
}


impl Solution {
    pub fn sorted_squares2(mut nums: Vec<i32>) -> Vec<i32> {
        nums.iter_mut()
            .filter(|&&mut x| x < 0)
            .for_each(|x| *x = -*x);
        let low = nums.iter().enumerate()
            .fold((0, nums[0]), |(idx, v), (jdx, &x)| {
                if x < v { (jdx, x) } else { ( idx, v ) }
            }).0;
        if low == 0 { return nums.iter().map(|x| x**x).collect() }
        let mut left = low as isize - 1;
        let mut right = low + 1;
        let n = nums.len();
        let mut v = Vec::with_capacity(n);
        v.push(nums[low]);
        // left -> 还需要处理的左侧第一个元素
        // right 还需要处理的右侧第一个元素
        while left >= 0 && right < n {
            // 判断两侧
            if nums[left as usize] < nums[right] {
                v.push(nums[left as usize]);
                left -= 1;
            } else {
                v.push(nums[right]);
                right += 1;
            }
        }
        if left < 0 {
            while right < n {
                v.push(nums[right]);
                right += 1;
            }
        } else {
            while left >= 0 {
                v.push(nums[left as usize]);
                left -= 1;
            }
        }
        v.iter().map(|x| x**x).collect()
    }
}


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut a = 0;
        let mut b = 0;
        let n = nums.len();
        while b < n {
            if nums[b] == val { b += 1 }
            else {
                nums[a] = nums[b];
                a += 1;
                b += 1;
            }
        }
        a as i32
    }
    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        if n == 0 { return 0 }
        // [i,j) 表示还需要处理的区间，在这个区间之外的都是无需处理的
        let mut i = 0;
        let mut j = n;
        while i < j {
            if nums[i] == val {
                j -= 1;
                nums[i] = nums[j];
            } else {
                i += 1;
            }
        }
        j as i32
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0isize;
        let mut right = nums.len() as isize - 1;
        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            if nums[mid] < target {
                left = mid as isize + 1
            } else if nums[mid] > target {
                right = mid as isize - 1;
            } else {
                return mid as i32
            }
        }
        -1
    }
}

impl Solution {
    pub fn search1(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32
            }
        }
        -1
    }
}

impl Solution {
    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        let x = nums.partition_point(|&x| x < target);
        if x == nums.len() { return -1 }
        if nums[x] == target { return x as i32 }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
        assert_eq!(Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    }
}