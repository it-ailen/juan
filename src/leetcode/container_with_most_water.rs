use super::dp::Problem;


struct Solution (usize);

impl Problem<Vec<i32>, i32> for Solution {
    fn id() -> usize {
        11
    }

    fn name() -> &'static str {
        "盛最多水的容器"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/container-with-most-water/"
    }

    fn solution(height: Vec<i32>) -> i32 {
        let mut l = 0usize;
        let mut r = height.len() - 1;
        let mut max = 0;
        while l < r {
            let area = std::cmp::min(height[l], height[r]) * (r - l) as i32;
            if area > max {
                max = area;
            }
            if height[l] < height[r] {
                l+=1;
            } else {
                r-=1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{container_with_most_water::Solution, dp::Problem};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![1,1]), 1);
        assert_eq!(Solution::solution(vec![4,3,2,1,4]), 16);
        assert_eq!(Solution::solution(vec![1,2,1]), 2);
        assert_eq!(Solution::solution(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}