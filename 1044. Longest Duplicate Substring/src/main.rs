fn main() {
    Solution::longest_dup_substring(String::from("moplvidmaagmsiyyrkchbyhivlqwqsjcgtumqscmxrxrvwsnjjvygrelcbjgbpounhuyealllginkitfaiviraqcycjmskrozcdqylbuejrgfnquercvghppljmojfvylcxakyjxnampmakyjbqgwbyokaybcuklkaqzawageypfqhhasetugatdaxpvtevrigynxbqodiyioapgxqkndujeranxgebnpgsukybyowbxhgpkwjfdywfkpufcxzzqiuglkakibbkobonunnzwbjktykebfcbobxdflnyzngheatpcvnhdwkkhnlwnjdnrmjaevqopvinnzgacjkbhvsdsvuuwwhwesgtdzuctshytyfugdqswvxisyxcxoihfgzxnidnfadphwumtgdfmhjkaryjxvfquucltmuoosamjwqqzeleaiplwcbbxjxxvgsnonoivbnmiwbnijkzgoenohqncjqnckxbhpvreasdyvffrolobxzrmrbvwkpdbfvbwwyibydhndmpvqyfmqjwosclwxhgxmwjiksjvsnwupraojuatksjfqkvvfroqxsraskbdbgtppjrnzpfzabmcczlwynwomebvrihxugvjmtrkzdwuafozjcfqacenabmmxzcueyqwvbtslhjeiopgbrbvfbnpmvlnyexopoahgmwplwxnxqzhucdieyvbgtkfmdeocamzenecqlbhqmdfrvpsqyxvkkyfrbyolzvcpcbkdprttijkzcrgciidavsmrczbollxbkytqjwbiupvsorvkorfriajdtsowenhpmdtvamkoqacwwlkqfdzorjtepwlemunyrghwlvjgaxbzawmikfhtaniwviqiaeinbsqidetfsdbgsydkxgwoqyztaqmyeefaihmgrbxzyheoegawthcsyyrpyvnhysynoaikwtvmwathsomddhltxpeuxettpbeftmmyrqclnzwljlpxazrzzdosemwmthcvgwtxtinffopqxbufjwsvhqamxpydcnpekqhsovvqugqhbgweaiheeicmkdtxltkalexbeftuxvwnxmqqjeyourvbdfikqnzdipmmmiltjapovlhkpunxljeutwhenrxyfeufmzipqvergdkwptkilwzdxlydxbjoxjzxwcfmznfqgoaemrrxuwpfkftwejubxkgjlizljoynvidqwxnvhngqakmmehtvykbjwrrrjvwnrteeoxmtygiiygynedvfzwkvmffghuduspyyrnftyvsvjstfohwwyxhmlfmwguxxzgwdzwlnnltpjvnzswhmbzgdwzhvbgkiddhirgljbflgvyksxgnsvztcywpvutqryzdeerlildbzmtsgnebvsjetdnfgikrbsktbrdamfccvcptfaaklmcaqmglneebpdxkvcwwpndrjqnpqgbgihsfeotgggkdbvcdwfjanvafvxsvvhzyncwlmqqsmledzfnxxfyvcmhtjreykqlrfiqlsqzraqgtmocijejneeezqxbtomkwugapwesrinfiaxwxradnuvbyssqkznwwpsbgatlsxfhpcidfgzrc"));
    // Solution::longest_dup_substring(String::from("hjkl"));


}

struct Solution {}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        if s.len() == 0 { return "".to_string() }
        let mut l_max = s.len() - 1;    // inclusive
        let mut l_min = 2;
        while l_min <= l_max {
            let l = if l_max == l_min + 1 { l_max } else { (l_max + l_min) / 2 };
            // println!("l_min = {:#?}, l_max = {:#?}", l_min, l_max);
            // println!("l = {:#?}", l);
            if let Some(substr) = Self::test(&s, l) {
                l_min = l;
                if l_max == l_min { return substr.to_owned() }
            } else {
                l_max = l - 1;
            }
        }
        "".to_string()
    }

    pub fn test(s_: &str, l: usize) -> Option<&str> {
        let s: Vec<usize> = s_.chars().map(|x| x as usize - 'a' as usize).collect();
        use std::collections::HashSet;
        let mut set: HashSet<usize> = HashSet::new();
        let modu = 2usize.pow(63) - 1;
        let p = Self::pow(26, l, modu);
        let mut num = s.iter().take(l).fold(0, |sum, x| (sum*26 + x)%modu);
        set.insert(num);
        for tail in l..s.len() {
            num = (num*26 + s[tail] - s[tail-l]*p)%modu;
            if !set.insert(num) { return Some(&s_[tail-l+1..=tail]) }
        }
        None
    }

    pub fn pow(x: usize, mut y: usize, modu: usize) -> usize {
        let mut res = x;
        while y > 1 {
            res *= x;
            y -= 1;
            if res > modu { res -= modu }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_dup_substring(String::from("ababa")), String::from("aba"));
        assert_eq!(Solution::longest_dup_substring(String::from("abcd")), String::from(""));
        assert_eq!(Solution::longest_dup_substring(String::from("abcdab")), String::from("ab"));
        assert_eq!(Solution::longest_dup_substring(String::from("abc")), String::from(""));
        assert_eq!(Solution::longest_dup_substring(String::from("")), String::from(""));
        assert_eq!(Solution::longest_dup_substring(String::from()), String::from(""));

    }

    #[test]
    fn pow() {
        assert_eq!(Solution::pow(2, 2, 3), 1);
        assert_eq!(Solution::pow(26, 3, 33333333333), 17576);

    }
}