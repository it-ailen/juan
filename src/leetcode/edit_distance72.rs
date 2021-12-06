use super::dp::Problem;

struct Solution(usize);

impl Solution {
    #[inline]
    fn swap<T>(left: T, right: T) -> (T, T) {
        (right, left)
    }
}

impl Problem<(String, String), i32> for Solution {
    fn solution(i: (String, String)) -> i32 {
        let word1 = i.0;
        let word2 = i.1;
        let len1 = word1.len();
        let len2 = word2.len();
        if len1 * len2 == 0 {
            return (len1 + len2) as i32;
        }
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();
        // min_dis[m][n] 表示分别以 word1[:m]、word2[:n] 结尾的子字符串的最小编辑距离
        let mut min_dis = vec![vec![usize::MAX; len2+1]; len1+1];
        for m in 0..=len1 {
            min_dis[m][0] = m;
        }
        for n in 0..=len2 {
            min_dis[0][n] = n;
        }
        for m in 1..=len1 {
            for n in 1..=len2 {
                let mut dis = min_dis[m - 1][n - 1] + if bytes1[m-1] == bytes2[n-1] { 0 } else { 1 };
                dis = std::cmp::min(dis, 1 + std::cmp::min(min_dis[m][n - 1], min_dis[m - 1][n]));
                min_dis[m][n] = dis;
            }
        }
        min_dis[len1][len2] as i32
    }

    fn id() -> usize {
        72
    }

    fn name() -> &'static str {
        "编辑距离"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/edit-distance/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, edit_distance72::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(("horse".to_string(), "ros".to_string())),
            3
        );
        assert_eq!(
            Solution::solution(("intention".to_string(), "execution".to_string())),
            5
        );
        assert_eq!(
            Solution::solution(("eat".to_string(), "sea".to_string())),
            2
        );
    }
}
