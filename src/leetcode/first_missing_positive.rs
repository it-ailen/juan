use super::dp::Problem;

struct Solution(usize);

impl Problem<Vec<i32>, i32> for Solution {
    fn solution(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut hits = vec![false; nums_len];
        for n in nums {
            if n > 0 && n <= nums_len as i32 {
                hits[n as usize - 1] = true;
            }
        }
        let mut missing = nums_len as i32 +1;
        for (idx, &found) in hits.iter().enumerate() {
            if !found {
                missing = idx as i32+1;
                break;
            }
        }
        missing
    }

    fn id() -> usize {
        41
    }

    fn name() -> &'static str {
        "缺失的第一个正数"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/first-missing-positive/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, first_missing_positive::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![1, 2, 0]), 3);
        assert_eq!(Solution::solution(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::solution(vec![7, 8, 9, 11, 12]), 1);
    }
}
