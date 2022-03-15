use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if num_map.contains_key(&(target - x)) {
                return vec![num_map[&(target - x)], i as i32];
            } else {
                num_map.insert(x, i as i32);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4, 8], 6));
    }
}
