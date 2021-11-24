use super::dp::Problem;

struct Solution(usize);

impl Problem<Vec<i32>, Vec<Vec<i32>>> for Solution {
    fn solution(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums.clone();
        nums.sort();
        let len = nums.len();
        let mut results = Vec::new();
        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = len - 1;
            while left < right {
                if nums[left] + nums[right] + nums[i] < 0 {
                    left += 1;
                } else if nums[left] + nums[right] + nums[i] > 0 {
                    right -= 1;
                } else {
                    results.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left - 1] == nums[left] {
                        left += 1
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1
                    }
                }
            }
        }
        results
    }

    fn id() -> usize {
        15
    }

    fn name() -> &'static str {
        "三数之和"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/3sum/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, threesum::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::solution(vec![0, 0, 0, 0, 0]),
            vec![vec![0, 0, 0,]]
        );
    }
}
