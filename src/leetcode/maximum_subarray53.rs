use super::dp::Problem;


struct Solution(usize);

impl Problem<Vec<i32>, i32> for Solution {
    fn solution(nums: Vec<i32>) -> i32 {
        let cnt = nums.len();
        // tail[i] 表示以 nums[i] 结尾的最大连续子序列和; 默认都为当前位置值。
        let mut tail = nums.clone();
        let mut max = nums[0];
        // 由于只关注 tail[idx-1]，所以可以用一个 prev 来记录即可，不需要整个数组空间；不过
        // 从算法思路来说，用数组较容易表达动态规划思想，所以并不需要对其进行优化
        for (idx, &i) in nums.iter().enumerate().skip(1) {
            if tail[idx-1] > 0 {
                tail[idx] += tail[idx-1];
            }
            if tail[idx] > max {
                max = tail[idx];
            }
        }
        max   
    }

    fn id() -> usize {
        53
    }

    fn name() -> &'static str {
        "最大子序和"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/maximum-subarray/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, maximum_subarray53::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(Solution::solution(vec![-2,1,-3,4,-1,2,1,5,4]), 15);
        assert_eq!(Solution::solution(vec![-2]), -2);
    }
}