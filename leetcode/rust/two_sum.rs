use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut values: HashMap<i32, i32> = HashMap::new();
        
        let size: i32 = nums.len().try_into().unwrap();

        for i in 0..size {
            if values.contains_key(&(target - nums[i as usize])) {
                return vec![values[&(target - nums[i as usize])], i as i32]
            }
            values.insert(nums[i as usize], i);
        }
        vec![]
    }
}