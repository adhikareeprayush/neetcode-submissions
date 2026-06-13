impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();

        for s in strs {
            result.push_str(&format!("{}#{}",s.len(),s))
        }

        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let mut j = i;

            while chars[j] != '#' {
                j +=1 ;
            }

            let len: usize = chars[i..j]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

            let word : String = chars[j + 1..j + 1+len]
                .iter()
                .collect();
            
            result.push(word);
            i = j + 1 + len;
        }
        result
    }
}
