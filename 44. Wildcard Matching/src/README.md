
## first thought

2d dp

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        let mut s: Vec<char> = s.chars().collect();
        s.insert(0, ' ');
        let mut p: Vec<char> = p.chars().collect();
        p.insert(0, ' ');
        println!("dp = {:#?}", dp);
        println!("s = {:#?}, p = {:#?}", s, p);
        for j in 1..p.len() {
            if p[j] != '*' { break }
            else { dp[0][j] = true }
        }
        for j in 1..p.len() {
            for i in 1..s.len() {
                if p[j] == '*' { dp[i][j] = dp[i-1][j-1] || dp[i][j-1] ||dp[i-1][j] }
                else if p[j] == '?' { dp[i][j] = dp[i-1][j-1] }
                else if s[i] == p[j] { dp[i][j] = dp[i-1][j-1] }
            }
        }
        println!("dp = {:#?}", dp);
        dp[s.len() - 1][p.len() - 1]
    }
}
```

the complexity will not be smaller than O(MN)

by the way, this 2d dp can be reduced to 1d dp, which will save some space but will not reduce time complexity.

## [a smarter way](http://yucoding.blogspot.com/2013/02/leetcode-question-123-wildcard-matching.html)

```rust 
pub fn is_match_g(s: String, p: String) -> bool {
    if s.is_empty() && p.is_empty() { return true }
    else if !s.is_empty() && p.is_empty() { return false }
    let s: Vec<char> = s.chars().collect();
    let mut p: Vec<char> = p.chars().collect();
    if s.is_empty() {
        p.dedup();
        if p == vec!['*'] { return true }
        else { return false }
    }
    // now s and p are both not empty
    let (mut i, mut j) = (0, 0);
    let (mut is, mut js) = (None, None);  // save i j 
    while i < s.len() {
        if j < p.len() && (p[j] == '?' || s[i] == p[j]) {
            i += 1;
            j += 1;
        } else if j < p.len() && p[j] == '*' {
            is = Some(i);
            js = Some(j);
            j += 1;
        } else if js != None {
            i = is.unwrap() + 1;
            is = Some(i);
            j = js.unwrap() + 1;
        } else {
            return false
        }
    }
    while j < p.len() && p[j] == '*' { j += 1 }
    if j == p.len() && p[j-1] == '*' { return true }
    if j == p.len() { true } else { false }
}
```

this approach's time complexity will be smaller than O(MN), at most O(MN).