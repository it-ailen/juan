use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn usize_sub(l: usize, r: usize) -> usize {
        if l >= r {
            l - r
        } else {
            r - l
        }
    }
    /// choices: 已作出的选择，
    /// return:
    fn backtrack(row: usize, n: usize, choices: &mut Vec<usize>, results: &mut Vec<Vec<usize>>) {
        let valid = |cur: usize| -> bool {
            for (r, &c) in choices.iter().enumerate() {
                if c as i32 - r as i32 == cur as i32 - row as i32 {
                    return false;
                }
                if cur == c || (c + r) == (cur + row) {
                    return false;
                }
            }
            true
        };
        // let mut used = vec![false; n];
        // for &c in choices.iter() {
        //     used[c] = true;
        // }
        if row == n - 1 {
            // 最后一行
            for i in 0..n {
                if valid(i) {
                    let mut solution = choices.clone();
                    solution.push(i);
                    results.push(solution);
                    break;
                }
            }

            return;
        }
        let mut cur_chosen = choices.clone();
        for i in 0..n {
            if !valid(i) {
                continue;
            }
            // 选择
            cur_chosen.push(i);
            Self::backtrack(row + 1, n, &mut cur_chosen, results);
            cur_chosen.pop();
        }
    }

    fn pos_2_string(pos: &Vec<usize>) -> Vec<String> {
        let len = pos.len();
        let mut results: Vec<String> = Vec::new();
        for &p in pos {
            let mut chars = vec!['.'; len];
            chars[p] = 'Q';
            results.push(chars.iter().collect());
        }
        results
    }
}

impl Problem<i32, Vec<Vec<String>>> for Solution {
    fn solution(n: i32) -> Vec<Vec<String>> {
        let mut chosen: Vec<usize> = Vec::new();
        let mut results: Vec<Vec<usize>> = Vec::new();
        Self::backtrack(0, n as usize, &mut chosen, &mut results);
        let mut sol: Vec<Vec<String>> = Vec::new();
        for pos in results {
            sol.push(Self::pos_2_string(pos.as_ref()));
        }
        // results.iter().map(|pos| {
        //     sol.push(Self::pos_2_string(pos))
        // });
        sol
    }

    fn id() -> usize {
        51
    }

    fn name() -> &'static str {
        "N 皇后"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/n-queens/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, n_queens::Solution};

    #[test]
    fn test() {
        // assert_eq!(Solution::solution(1), vec![vec!["Q".to_string()]]);
        assert_eq!(
            Solution::solution(4),
            vec![
                vec![".Q..".to_string(), "...Q".to_string(), "Q...".to_string(), "..Q.".to_string()],
                vec!["..Q.".to_string(), "Q...".to_string(), "...Q".to_string(), ".Q..".to_string()],
            ]
        );
    }
}
