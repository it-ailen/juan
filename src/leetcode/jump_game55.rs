use super::dp::Problem;

struct Solution(usize);

impl Problem<Vec<i32>, bool> for Solution {
    fn solution(nums: Vec<i32>) -> bool {
        let len = nums.len();
        // far_index[i] 表示以 i 结尾的子数组中，能达到的最远下标
        // let mut far_index = vec![0; len];
        // 初始最远能到达 0
        let mut cur_max_index = 0;
        for (idx, &n) in nums.iter().enumerate() {
            if idx > cur_max_index {
                break;
            }
            let max = std::cmp::max(cur_max_index, n as usize + idx);
            if max > cur_max_index {
                cur_max_index = max;
            }
        }
        cur_max_index >= len-1
    }

    fn id() -> usize {
        55
    }

    fn name() -> &'static str {
        "跳跃游戏"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/jump-game/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, jump_game55::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::solution(vec![0]), true);
        assert_eq!(Solution::solution(vec![3, 2, 1, 0, 4]), false);
    }
}
