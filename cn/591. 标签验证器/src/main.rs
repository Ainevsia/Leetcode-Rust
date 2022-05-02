fn main() {
    Solution::is_valid("<DIV><AB>123</AB></DIV>".to_string());
}

struct Solution {}

impl Solution {
    pub fn is_valid(code: String) -> bool {
        let code = if let Some(res) = Self::check_tag(&code) { res } else { return false };
        let codev = code.chars().collect::<Vec<char>>();
        let mut stack: Vec<&str> = vec![];
        let cdata1 = "<![CDATA[";   // 9
        let cdata2 = "]]>";         // 3
        let len1 = cdata1.len();
        let len2 = cdata2.len();
        let n = code.len();
        let mut i = 0;
        while i < n {
            if i + len1 - 1 < n && code[i..i+len1].eq(cdata1) {
                let mut j = i + len1;
                let mut found_tail = false;
                while j < n && !found_tail {
                    if j + len2 - 1 < n && code[j..j+len2].eq(cdata2) {
                        found_tail = true
                    } else { j += 1 }
                }
                if !found_tail { return false }
                i = j + len2;
            } else if codev[i] == '<' {
                if i + 1 >= n { return false }
                let is_end = codev[i+1] == '/';
                let idx = if let Some(x) = codev[i..].iter()
                    .position(|&x| x == '>') { x } else { return false };
                    let tag = if is_end { &code[i+2..i+idx] } else { &code[i+1..i+idx] };
                // check upper case
                if ! tag.chars().all(|x| x.is_ascii_uppercase()) { return false }
                if tag.len() > 9 || tag.len() < 1 { return false }
                if is_end {
                    if let Some(x) = stack.pop() {
                        if !x.eq(tag) { return false }
                    } else { return false };
                } else {
                    stack.push(tag);
                }
                i += idx + 1;
            } else {
                i += 1;
            }
        }
        stack.is_empty()
    }

    fn check_tag(s: &str) -> Option<String> {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        if s[0] != '<' { return None }
        let idx = if let Some(idx) = s.iter().position(|&x| x == '>') {
            idx
        } else { return None };
        if idx == 1 { return None };
        let tag = &s[1..idx];
        let tag_len = tag.len();
        // max len
        if tag_len < 1 || tag_len > 9 { return None };
        // valid tag all upper case letters
        if ! tag.iter().all(|x| x.is_ascii_uppercase()) { return None }
        // min length
        if n < (tag_len + 2) * 2 + 1 { return None }
        // check tail
        if ! tag.iter().eq(s[n-tag_len-1..n-1].iter()) { return None }
            if s[n-1] != '>' || s[n-tag_len-2] != '/' || s[n-tag_len-3] != '<' { return None }
        Some(s[tag_len+2..n-tag_len-3].iter().collect::<String>())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_valid("<DIV><AB>123</AB></DIV>".to_string()),true);
        assert_eq!(Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),true);
        assert_eq!(Solution::is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()),true);
        assert_eq!(Solution::is_valid("<A>  <B> </A>   </B>".to_string()),false);
        assert_eq!(Solution::is_valid("<DIV>  div tag is not closed  <DIV>".to_string()),false);
        assert_eq!(Solution::is_valid("<DIV>  unmatched <  </DIV>".to_string()),false);
        assert_eq!(Solution::is_valid("<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_string()),false);
        assert_eq!(Solution::is_valid("<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>".to_string()),false);
        assert_eq!(Solution::is_valid("<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>".to_string()),false);
        assert_eq!(Solution::is_valid("<AAAAAAAAAA></AAAAAAAAAA>".to_string()),false);
        assert_eq!(Solution::is_valid("<A></A>".to_string()),true);
        assert_eq!(Solution::is_valid("<A><B><![CDATA[></B>]]></A>".to_string()),false);
    }
}