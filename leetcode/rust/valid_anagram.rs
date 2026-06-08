// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (s.len() != t.len()) {
            return false;
        }

        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();

        let n: usize = s_vec.len();

        let mut map1: HashMap<char, usize> = HashMap::new();
        let mut map2: HashMap<char, usize> = HashMap::new();

        for i in 0..n {
            let char1 = s_vec[i];
            let char2 = t_vec[i];

            *map1.entry(char1).or_insert(0) += 1;
            *map2.entry(char2).or_insert(0) += 1;
        }

        if (map1 == map2) {
            return true;
        }
        return false;
    }
}