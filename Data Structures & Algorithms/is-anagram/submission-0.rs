use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut charSet1 = HashMap::new();
        let mut charSet2 = HashMap::new();

        for c in s.chars() {
            *charSet1.entry(c).or_insert(0) +=1;
        }
        for c in t.chars() {
            *charSet2.entry(c).or_insert(0) +=1;
        }

        charSet1 == charSet2
    }
}
