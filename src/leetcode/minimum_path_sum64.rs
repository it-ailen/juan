use super::dp::Problem;

struct Solution(usize);

impl Problem<Vec<Vec<i32>>, i32> for Solution {
    fn solution(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        if row == 0 {
            return 0;
        }
        let col = grid[0].len();
        if col == 0 {
            return 0;
        }
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; col]; row];
        memo[0][0] = grid[0][0];
        for r in 1..row {
            memo[r][0] = memo[r-1][0] + grid[r][0];
        }
        for c in 1..col {
            memo[0][c] = memo[0][c-1] + grid[0][c];
        }
        for r in 1..row {
            for c in 1..col {
                memo[r][c] = grid[r][c] + std::cmp::min(memo[r-1][c], memo[r][c-1]);
            }
        }
        memo[row-1][col-1]
    }

    fn id() -> usize {
        64
    }

    fn name() -> &'static str {
        "最小路径和"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/minimum-path-sum/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, minimum_path_sum64::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
        assert_eq!(Solution::solution(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
