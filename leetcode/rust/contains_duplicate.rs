// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for i in nums {
            if set.contains(&i) {
                return true;
            }
            set.insert(i);
        }
        return false;
    }
}