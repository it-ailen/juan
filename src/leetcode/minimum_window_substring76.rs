use std::collections::{HashMap, HashSet};

use super::dp::Problem;

struct Solution(usize);

impl Solution {
    fn cover(target: &[u8; 256], window: &[u8; 256], hit: &Vec<u8>) -> bool {
        for &c in hit {
            let c = c as usize;
            if window[c] < target[c] {
                return false;
            }
        }
        true
    }

    fn implement1(s: String, t: String) -> String {
        if s.len() < t.len() || t.len() == 0 {
            return "".to_string();
        }
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let mut char_set: Vec<u8> = Vec::new();
        let mut tb_tbl = [0u8; 256];
        for &b in tb {
            tb_tbl[b as usize] += 1;
            if tb_tbl[b as usize] == 1 {
                char_set.push(b);
            }
        }
        let mut min_cover_len = usize::MAX;
        let mut min_cover_start = 0usize;
        let mut slow = 0usize;
        let mut fast = 0usize;
        let mut windown_tbl = [0u8; 256];
        while fast < s.len() {
            let cur = sb[fast] as usize;
            if tb_tbl[cur] > 0 {
                // 表示当前 c 是目标字符串中的字符
                windown_tbl[cur] += 1;
            }
            fast += 1;
            if Self::cover(&tb_tbl, &windown_tbl, &char_set) {
                while slow < fast {
                    let left = sb[slow] as usize;
                    if tb_tbl[left] == 0 {
                        // 不在 t 范围内，直接跳过
                        slow += 1;
                        continue;
                    }
                    if windown_tbl[left] > tb_tbl[left] {
                        slow += 1;
                        windown_tbl[left] -= 1;
                    } else {
                        break;
                    }
                }
                if fast - slow < min_cover_len {
                    min_cover_len = fast - slow;
                    min_cover_start = slow;
                }
                break;
            }
        }
        while fast < s.len() {
            let cur = sb[fast] as usize;
            fast += 1;
            if tb_tbl[cur] > 0 {
                windown_tbl[cur] += 1;
                while slow < fast {
                    let left = sb[slow] as usize;
                    if tb_tbl[left] == 0 {
                        // 不在 t 范围内，直接跳过
                        slow += 1;
                        continue;
                    }
                    if windown_tbl[left] > tb_tbl[left]
                    // 在 t 范围内，且有多余字符
                    {
                        slow += 1;
                        windown_tbl[left] -= 1;
                    } else {
                        break;
                    }
                }
                if fast - slow < min_cover_len {
                    min_cover_len = fast - slow;
                    min_cover_start = slow;
                }
            }
        }
        if min_cover_len < usize::MAX {
            String::from(&s[min_cover_start..min_cover_start + min_cover_len])
        } else {
            "".to_string()
        }
    }

    fn check_cover(s: &HashMap<char, i32>) -> bool {
        for (_, &v) in s.iter() {
            if v < 0 {
                return false;
            }
        }
        true
    }

    fn implement2(s: String, t: String) -> String {
        if s.len() < t.len() || t.len() == 0 {
            return "".to_string();
        }
        let mut char_set: HashSet<char> = HashSet::new();
        let mut target_chars: HashMap<char, i32> = HashMap::new();
        let t_bytes = t.as_bytes();
        for &c in t_bytes {
            *target_chars.entry(c as char).or_insert(0) -= 1;
            char_set.insert(c as char);
        }
        let s_bytes = s.as_bytes();
        let mut slow = 0usize;
        let mut fast = 0usize;
        let mut res: &str = &"";
        while fast < s_bytes.len() {
            let c = s_bytes[fast] as char;
            if char_set.contains(&c) {
                *target_chars.entry(c).or_insert(0) += 1;
                if Self::check_cover(&target_chars) {
                    while slow <= fast && Self::check_cover(&target_chars) {
                        let slow_c = s_bytes[slow] as char;
                        if char_set.contains(&slow_c) {
                            *target_chars.entry(slow_c).or_insert(0) -= 1;
                        }
                        slow += 1;
                    }
                    if fast - (slow - 1) + 1 < res.len() || res.len() == 0 {
                        res = &s[slow - 1..=fast];
                    }
                }
            }

            fast += 1;
        }
        String::from(res)
    }
}

impl Problem<(String, String), String> for Solution {
    fn solution(i: (String, String)) -> String {
        let s = i.0;
        let t = i.1;
        // Self::implement1(s, t)
        Self::implement2(s, t)
    }

    fn id() -> usize {
        76
    }

    fn name() -> &'static str {
        "最小覆盖子串"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/minimum-window-substring/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, minimum_window_substring76::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution(("ADOBECODEBANC".to_string(), "ABC".to_string())),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::solution(("a".to_string(), "a".to_string())),
            "a".to_string()
        );
        assert_eq!(
            Solution::solution(("a".to_string(), "aa".to_string())),
            "".to_string()
        );
        assert_eq!(
            Solution::solution(("ab".to_string(), "b".to_string())),
            "b".to_string()
        );
        // assert_eq!(
        //     Solution::solution(("abcdddscdsesa".to_string(), "dda".to_string())),
        //     "".to_string()
        // );
    }
}
