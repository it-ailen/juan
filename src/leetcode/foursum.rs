use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn sum(sorted: Vec<i32>, n: usize, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let len = sorted.len();
        if n == 0 {
            return Vec::new();
        } else if n == 1 {
            for &each in sorted.iter() {
                if each == target {
                    result.push(vec![each]);
                    break;
                } else if each > target {
                    break;
                }
            }
        } else {
            for i in (1..len).rev() {
                // println!("i=={:?}", i);
                if i < len - 1 && sorted[i] == sorted[i + 1] {
                    continue;
                }
                let mut sub = Self::sum(sorted[..i].to_vec(), n - 1, target - sorted[i]);
                for each in sub.iter_mut() {
                    each.push(sorted[i]);
                    result.push(each.to_owned());
                }
            }
        }
        result
    }
}

impl Problem<(Vec<i32>, i32), Vec<Vec<i32>>> for Solution {
    fn id() -> usize {
        18
    }

    fn name() -> &'static str {
        "4数之和"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/4sum/"
    }

    fn solution(i: (Vec<i32>, i32)) -> Vec<Vec<i32>> {
        let mut nums = i.0.clone();
        let target = i.1;
        nums.sort();
        Self::sum(nums, 4, target)
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, foursum::Solution};

    #[test]
    fn sum() {
        assert_eq!(Solution::sum(vec![-2, -1], 2, -3), vec![vec![-2, -1]]);
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution((vec![-1, 0, 1, 2, -1, -4], 2)),
            vec![vec![-1, 0, 1, 2]]
        );
        assert_eq!(
            Solution::solution((vec![2,2,2,2,2], 8)),
            vec![vec![2, 2, 2, 2]]
        );
        assert_eq!(
            Solution::solution((vec![1, 0, -1, 0, -2, 2], 0)),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }
}
