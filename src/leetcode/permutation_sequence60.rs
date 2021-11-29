use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn vec_2_string(chosen: &Vec<i32>) -> String {
        // let mut chars: Vec<char> = Vec::new();
        chosen
            .iter()
            .map(|&i| ('0' as u8 + i as u8) as char)
            .collect()
    }

    fn permutation(
        n: i32,
        chosen: &mut Vec<i32>,
        result_num: &mut i32,
        target: i32,
    ) -> Option<String> {
        for i in 1..=n {
            if chosen.contains(&i) {
                continue;
            }
            chosen.push(i); // 选择
            if chosen.len() == n as usize {
                *result_num += 1;
                if *result_num == target {
                    return Some(Self::vec_2_string(chosen));
                }
            } else {
                if let Some(s) = Self::permutation(n, chosen, result_num, target) {
                    return Some(s);
                }
            }
            chosen.pop(); // 回溯
        }
        None
    }
}

impl Problem<(i32, i32), String> for Solution {
    fn solution(i: (i32, i32)) -> String {
        let n = i.0;
        let k = i.1;
        let mut chosen = Vec::new();
        let mut result_num = 0;
        let a = Self::permutation(n, &mut chosen, &mut result_num, k);
        a.unwrap()
    }

    fn id() -> usize {
        60
    }

    fn name() -> &'static str {
        "排列序列"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/permutation-sequence/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, permutation_sequence60::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::solution((3, 1)), "123".to_string());
        assert_eq!(Solution::solution((3, 3)), "213".to_string());
        assert_eq!(Solution::solution((4, 9)), "2314".to_string());
    }
}