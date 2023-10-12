use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //let mut result = Vec::new();
        let mut indexes = HashMap::new();

        // Approach 1: brute force
        // for i in 0..nums.len() {
        //     for j in (i + 1)..nums.len() {
        //         if nums[i] + nums[j] == target {
        //             result.push(i as i32);
        //             result.push(j as i32);
        //             return result;
        //         }
        //     }
        // }

        // Approach 2: Two-pass hash table
        for i in 0..nums.len() {
            indexes.insert(nums[i], i);
        }

        for i in 0..nums.len() {
            if let Some(&j) = indexes.get(&(target - nums[i])) {
                if(i != j) {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        return vec!() // Return an empty vector if no solution is found
    }
}
