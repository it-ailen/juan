use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn spiral_round(
        nums: &Vec<Vec<i32>>,
        r_up: usize,
        r_down: usize,
        c_left: usize,
        c_right: usize,
    ) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        if r_up > r_down || c_left > c_right {
            return results;
        }
        for c in c_left..=c_right {
            results.push(nums[r_up][c]);
        }
        for r in (r_up + 1)..r_down {
            results.push(nums[r][c_right]);
        }
        if r_up < r_down {
            for c in (c_left..=c_right).rev() {
                results.push(nums[r_down][c]);
            }
        }
        if c_left < c_right {
            for r in ((r_up + 1)..r_down).rev() {
                results.push(nums[r][c_left]);
            }
        }

        if r_up < r_down && c_left < c_right {
            results.extend(Self::spiral_round(
                nums,
                r_up + 1,
                r_down - 1,
                c_left + 1,
                c_right - 1,
            ));
        }
        results
    }
}

impl Problem<Vec<Vec<i32>>, Vec<i32>> for Solution {
    fn solution(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let result: Vec<i32> = Vec::new();
        if matrix.len() == 0 {
            return result;
        }
        let row = matrix.len();
        let col = matrix[0].len();
        Self::spiral_round(&matrix, 0, row - 1, 0, col - 1)
    }

    fn id() -> usize {
        54
    }

    fn name() -> &'static str {
        "螺旋矩阵"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/spiral-matrix/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, spiral_matrix54::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::solution(vec![vec![1]]), vec![1]);
        assert_eq!(Solution::solution(vec![vec![1], vec![2]]), vec![1, 2]);
        assert_eq!(Solution::solution(vec![vec![1], vec![2], vec![3]]), vec![1, 2, 3]);

    }
}
