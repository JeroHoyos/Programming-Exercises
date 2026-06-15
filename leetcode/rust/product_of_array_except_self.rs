impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

        let size = nums.len();

        let mut prefix = vec![1; size];

        let mut suffix = vec![1; size];

        for i in 1..size {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        for i in (1..size).rev() {
            suffix[i - 1] = suffix[i] * nums[i];
        }

        let mut answer = vec![1; size];
        
        for i in 0..size {
            answer[i] = prefix[i] * suffix[i];
        }

        answer
    }
}