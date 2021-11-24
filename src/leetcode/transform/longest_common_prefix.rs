use crate::leetcode::dp::Problem;

struct Solution(usize);

impl Solution {
    fn common_prefix<'a>(l: &'a str, r: &'a str) -> String {
        let mut result = String::from("");
        let min = std::cmp::min(l.len(), r.len());
        let l = l.as_bytes();
        let r = r.as_bytes();
        for i in 0..min {
            if l[i] == r[i] {
                result.push(l[i] as char);
            } else {
                break;
            }
        }
        result
    }
}

impl Problem<Vec<String>, String> for Solution {
    fn solution(i: Vec<String>) -> String {
        if i.len() == 0 {
            return "".to_owned();
        } else if i.len() == 1 {
            return i[0].clone();
        }
        let mid = i.len() / 2;
        let left_prefix = Self::solution(i[..mid].to_vec());
        let right_prefix = Self::solution(i[mid..].to_vec());
        Self::common_prefix(&left_prefix, &right_prefix)
    }

    fn id() -> usize {
        14
    }

    fn name() -> &'static str {
        "最长公共前缀"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/longest-common-prefix/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, transform::longest_common_prefix::Solution};

    #[test]
    fn test_base() {
        assert_eq!(Solution::solution(vec![String::from("")]), "");
        assert_eq!(Solution::solution(vec![String::from(""), String::from("1")]), "");
        assert_eq!(Solution::solution(vec![String::from("123"), String::from("124")]), "12");
    }
}
