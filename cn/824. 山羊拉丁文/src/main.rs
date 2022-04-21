fn main() {
    Solution::to_goat_latin("I speak Goat Latin".to_owned());
}

struct Solution {}

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut res = vec![];
        for (idx, i) in sentence.split(' ').enumerate() {
            if i.to_lowercase().starts_with(['a', 'e', 'i', 'o', 'u']) {
                let x = format!("{}maa{}",i,"a".repeat(idx));
                res.push(x);
            } else {
                let x = format!("{}{}maa{}",&i[1..],&i[..1],"a".repeat(idx));
                res.push(x);
            }
        }
        // println!("res = {:#?}", res);
        res.join(" ")
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_owned()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
        assert_eq!(
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_owned()),
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
        );
        assert_eq!(
            Solution::to_goat_latin("Ii wefj weofi woe eofiwejf owefi jowe r speak Goat Latin".to_owned()),
            "Iimaa efjwmaaa eofiwmaaaa oewmaaaaa eofiwejfmaaaaaa owefimaaaaaaa owejmaaaaaaaa rmaaaaaaaaa peaksmaaaaaaaaaa oatGmaaaaaaaaaaa atinLmaaaaaaaaaaaa"
        );
    }
}