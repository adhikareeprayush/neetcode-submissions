impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let new_string : Vec<char> = s
            .trim()
            .chars() 
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase())
            .flatten()
            .collect();
        
        if new_string.len() == 0 || new_string.len() == 1 {
            return true;
        }

        let mut left = 0;
        let mut right = new_string.len()-1;

        while left < right {
            if new_string[left] != new_string[right] {
                return false;
            }
            left+=1;
            right-=1;
        }
        return true;
    }
}
