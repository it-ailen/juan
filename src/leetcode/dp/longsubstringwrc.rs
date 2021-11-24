use super::Problem;

/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
pub struct LongSubstringWRC {}

impl LongSubstringWRC {
    pub fn new() -> Self {
        Self {}
    }
}

impl Problem<String, i32> for LongSubstringWRC {
    fn solution(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let s = s.as_bytes();
        // 最后一次出现该 char 的下标位置
        let mut last_occur = [-1i32; 256];
        let mut last_index = vec![-1i32; s.len()];
        for (idx, &c) in s.iter().enumerate() {
            last_index[idx] = last_occur[c as usize];
            last_occur[c as usize] = idx as i32;
        }
        let mut memo = vec![1usize; s.len()];
        let mut max = 1;
        for (idx, _) in s.iter().enumerate().skip(1) {
            let last_pos = last_index[idx];
            let val: i32;
            if (idx - memo[idx - 1]) as i32 > last_pos {
                val = (memo[idx - 1] + 1) as i32;
            } else {
                val = (idx as i32 - last_pos) as i32;
            }
            memo[idx] = val as usize;
            if val > max {
                max = val;
            }
        }
        max
    }

    fn id() -> usize {
        3
    }

    fn name() -> &'static str {
        "无重复字符的最长子串"
    }

    fn url() -> &'static str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::dp::{Problem, longsubstringwrc::LongSubstringWRC};

    #[test]
    fn base() {
        assert_eq!(LongSubstringWRC::solution("".to_string()), 0);
        assert_eq!(LongSubstringWRC::solution("s".to_string()), 1);
        assert_eq!(LongSubstringWRC::solution("au".to_string()), 2);
        assert_eq!(LongSubstringWRC::solution("bbbbb".to_string()), 1);
        assert_eq!(LongSubstringWRC::solution("abcabcbb".to_string()), 3);
        assert_eq!(LongSubstringWRC::solution("pwwkew".to_string()), 3);
    }
}
