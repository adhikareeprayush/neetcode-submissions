use::std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seqSet = HashSet::new();
        
        for num in nums {
            seqSet.insert(num);
        }

        let mut longest = 0;
        
        for &num in &seqSet {
            if !seqSet.contains(&(num - 1)) {
                let mut current = num;
                let mut length = 1;

                while seqSet.contains(&(current + 1)) {
                    current += 1;
                    length += 1; 
                }
                longest = longest.max(length);
            }
        }

        longest
    }
}
