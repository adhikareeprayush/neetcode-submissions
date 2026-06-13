impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqMap : HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            *freqMap.entry(nums[i]).or_insert(0) +=1;
        }

        let mut vec : Vec<(i32, i32)> = freqMap.into_iter().collect();
        vec.sort_by(|a,b| b.1.cmp(&a.1)); 

        vec.into_iter() 
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()

    }
}
