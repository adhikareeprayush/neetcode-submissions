use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map : HashMap<String, Vec<String>> = HashMap::new(); // "abc" -> ["acb", "bac"]
        
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect(); // ['a', 'b', 'c']
            chars.sort_unstable();
            let key : String = chars.into_iter().collect(); // "abc"
            
            map.entry(key)
            .or_default()
            .push(s);
        }
    
        map.into_values().collect()   
    }
}
