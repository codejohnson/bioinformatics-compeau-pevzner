    //challenge 1A, Bioinformatics Compeau-Pevzner
    pub fn pattern_count(text:&String,pttrn:String) -> usize {
        if text.len() < pttrn.len() {return 0}
        let mut c:usize = 0;
        for i in 0..=text.len() - pttrn.len() {
            c += (text[i..i+pttrn.len()] == pttrn) as usize;
        }
        c
    }

    //challenge 1B, Bioinformatics Compeau-Pevzner
    use std::collections::HashSet;
    pub fn frequent_words(text:String,k:usize) -> HashSet<String> {
        let mut frequent_patterns:HashSet<String> = HashSet::new();
        let n = text.len() - k + 1;
        let mut count = vec![0;n];
        for i in 0..n {
            count[i] = pattern_count(&text, text[i..i+k].to_string());
        }
        let max_count = count.iter().max().unwrap();
        for i in 0..n {
            if count[i] == *max_count {
                frequent_patterns.insert(text[i..i+k].to_string());
            }
        }
        return frequent_patterns
    }
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/