impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut arr : Vec<i32> = nums.clone();
        arr.sort();
        for i in 0..arr.len().saturating_sub(1) {
            if arr[i] == arr[i+1] {
                return true;
            }
        }
        return false;
    }
}
