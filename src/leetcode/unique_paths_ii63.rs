use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn compute(grid: &Vec<Vec<i32>>, target_r: usize, target_c: usize, memo: &mut Vec<Vec<i32>>) {
        if memo[target_r][target_c] >= 0 {
            return;
        }
        if grid[target_r][target_c] == 1 {
            memo[target_r][target_c] = 0;
            return;
        }
        let mut step = 0;
        if target_r > 0 && grid[target_r - 1][target_c] != 1 {
            if memo[target_r - 1][target_c] < 0 {
                Self::compute(grid, target_r - 1, target_c, memo);
            }
            if memo[target_r - 1][target_c] > 0 {
                step += memo[target_r - 1][target_c];
            }
        }
        if target_c > 0 && grid[target_r][target_c - 1] != 1 {
            if memo[target_r][target_c - 1] < 0 {
                Self::compute(grid, target_r, target_c - 1, memo);
            }
            if memo[target_r][target_c - 1] > 0 {
                step += memo[target_r][target_c - 1];
            }
        }
        memo[target_r][target_c] = step;
    }
}

impl Problem<Vec<Vec<i32>>, i32> for Solution {
    fn solution(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row = obstacle_grid.len();
        if row == 0 {
            return 0;
        }

        let col = obstacle_grid[0].len();
        let mut memo = vec![vec![-1; col]; row];
        if obstacle_grid[0][0] == 0 {
            memo[0][0] = 1;
        }
        Self::compute(&obstacle_grid, row - 1, col - 1, &mut memo);
        memo[row - 1][col - 1]
    }

    fn id() -> usize {
        63
    }

    fn name() -> &'static str {
        "不同路径 II"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/unique-paths-ii/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, unique_paths_ii63::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );
        assert_eq!(Solution::solution(vec![vec![0, 1], vec![0, 0]]), 1);
        assert_eq!(Solution::solution(vec![vec![1]]), 0);
        assert_eq!(Solution::solution(vec![vec![1, 0]]), 0);
        assert_eq!(Solution::solution(vec![vec![0]]), 1);
    }
}
