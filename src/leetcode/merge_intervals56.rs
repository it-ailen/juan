use core::cmp::Ordering;

use super::dp::Problem;

// type Interval = Vec<i32>;
struct Solution(usize);

impl Problem<Vec<Vec<i32>>, Vec<Vec<i32>>> for Solution {
    fn solution(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return intervals.clone();
        }
        intervals.sort_by(|a, b| {
            if a[0] < b[0] {
                Ordering::Less
            } else if a[0] == b[0] {
                if a[1] < b[1] {
                    Ordering::Less
                } else if a[1] == b[1] {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Greater
            }
        });
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut last_merged_interval = intervals[0].clone();
        for i in 1..intervals.len() {
            let cur_start = intervals[i][0];
            let cur_end = intervals[i][1];
            if cur_start > last_merged_interval[1] {
                results.push(last_merged_interval.clone());
                last_merged_interval = intervals[i].clone();
            } else {
                last_merged_interval[1] = std::cmp::max(last_merged_interval[1], cur_end);
            }
        }
        results.push(last_merged_interval);
        results
    }

    fn id() -> usize {
        56
    }

    fn name() -> &'static str {
        "合并区间"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/merge-intervals/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, merge_intervals56::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::solution(vec![vec![1, 3], vec![3, 6]]),
            vec![vec![1, 6]]
        );
        assert_eq!(Solution::solution(vec![vec![1, 3]]), vec![vec![1, 3]]);
    }
}
