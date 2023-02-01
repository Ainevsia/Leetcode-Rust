pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let s1 = sentence1.split(' ').collect::<Vec<_>>();
    let s2 = sentence2.split(' ').collect::<Vec<_>>();
    if s1.starts_with(&s2) ||
       s2.starts_with(&s1) ||
       s1.ends_with(&s2)   ||
       s2.ends_with(&s1) {
        return true;
    }
    if sentence1.len() == sentence2.len() {
        return sentence1 == sentence2
    }
    let (s1, s2) = if sentence1.len() < sentence2.len() {
        (sentence2, sentence1)
    } else {
        (sentence1, sentence2)
    };
    // s1 longer
    let words = s2.split(' ').collect::<Vec<_>>();
    let longer = s1.split(' ').collect::<Vec<_>>();
    // dbg!(&words);
    for i in 1..words.len() {
        let f1 = &words[..i];
        let f2 = &words[i..];
        // dbg!(&f1, &f2);
        if longer.starts_with(f1) && longer.ends_with(f2) {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(are_sentences_similar("My name is Haley".into(),"My Haley".into()), true);
        assert_eq!(are_sentences_similar("of".into(),"A lot of words".into()), false);
        assert_eq!(are_sentences_similar("Eating right now".into(),"Eating".into()), true);
        assert_eq!(are_sentences_similar("Luky".into(),"Lucccky".into()), false);
        assert_eq!(are_sentences_similar("B".into(),"ByI BMyQIqce b bARkkMaABi vlR RLHhqjNzCN oXvyK zRXR q ff B yHS OD KkvJA P JdWksnH".into()), false);
    }
}
