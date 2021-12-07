use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn wrong(heights: Vec<i32>) -> i32 {
        if heights.len() == 0 {
            return 0;
        }
        let mut max: i32 = heights[0];
        let mut tail_max_area = vec![0; heights.len()];
        let mut last_area_len = 1usize;
        tail_max_area[0] = heights[0];
        for i in 1..heights.len() {
            let last_common_min_height = tail_max_area[i - 1] / last_area_len as i32;
            tail_max_area[i] = if heights[i] >= last_common_min_height {
                if heights[i] >= tail_max_area[i - 1] + last_common_min_height {
                    last_area_len = 1;
                    heights[i]
                } else {
                    last_area_len += 1;
                    tail_max_area[i - 1] + last_common_min_height
                }
            } else {
                last_area_len += 1;
                heights[i] * (last_area_len as i32)
            };
            if tail_max_area[i] > max {
                max = tail_max_area[i];
            }
        }
        max
    }

    /// 暴力算法，会超时
    fn solution_force(heights: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..heights.len() {
            let mut area = heights[i];
            for j in (0..i).rev() {
                if heights[j] >= heights[i] {
                    area += heights[i];
                } else {
                    break;
                }
            }
            for j in (i + 1..heights.len()) {
                if heights[j] >= heights[i] {
                    area += heights[i];
                } else {
                    break;
                }
            }
            if area > max {
                max = area;
            }
        }
        max
    }

    /// 使用单调栈方式实现
    fn solution_stack(heights: Vec<i32>) -> i32 {
        // left_bound_index[i] 表示 i 左边可达到的第一个满足 heights[i] > heights[j] 的 j 值，-1表示标志位
        let mut left_bound_index = vec![-1; heights.len()];
        let mut stack: Vec<i32> = Vec::new();
        for i in 0..heights.len() {
            while stack.len() > 0 {
                let top = heights[stack[stack.len() - 1] as usize];
                if top < heights[i] {
                    break;
                }
                stack.pop();
            }
            left_bound_index[i] = if stack.len() == 0 {
                -1
            } else {
                stack[stack.len()-1]
            };
            stack.push(i as i32);
        }
        let mut right_bound_index = vec![heights.len(); heights.len()];
        let mut stack: Vec<usize> = Vec::new();
        for i in (0..heights.len()).rev() {
            while stack.len() > 0 {
                let top = heights[stack[stack.len()-1]];
                if top < heights[i] {
                    break;
                }
                stack.pop();
            }
            right_bound_index[i] = if stack.len() == 0 {
                heights.len()
            } else {
                stack[stack.len()-1]
            };
            stack.push(i);
        }
        let mut max = 0;
        for i in 0..heights.len() {
            let area = heights[i] * (right_bound_index[i] as i32 - left_bound_index[i] - 1);
            if area > max {
                max = area;
            }
        }
        max
    }
}

impl Problem<Vec<i32>, i32> for Solution {
    fn solution(heights: Vec<i32>) -> i32 {
        Self::solution_stack(heights)
    }

    fn id() -> usize {
        84
    }

    fn name() -> &'static str {
        "柱状图中最大的矩形"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/largest-rectangle-in-histogram/"
    }

    fn tags() -> Vec<&'static str> {
        vec!["单调栈", "哨兵技巧"]
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, largest_rectangle_in_histogram84::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::solution(vec![1, 2]), 2);
        assert_eq!(Solution::solution(vec![2, 1, 2]), 3);
        assert_eq!(Solution::solution(vec![1, 2, 2]), 4);
        assert_eq!(Solution::solution(vec![1]), 1);
        assert_eq!(Solution::solution(vec![]), 0);
    }
}
