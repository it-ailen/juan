/// https://leetcode-cn.com/problems/longest-palindromic-substring/
pub fn solution(s: String) -> String {
    if s.len() == 0 {
        return s;
    }
    let chars = s.as_bytes();
    let len = s.len();
    let mut max = 1;
    let mut max_start = 0;
    let mut flag: Vec<Vec<_>> = vec![vec![false; len]; len];
    for i in 0..len {
        flag[i][i] = true;
    }
    for i in (0..len - 1).rev() {
        for j in (i + 1)..len {
            let val: bool;
            if j == i + 1 {
                val = chars[i] == chars[j];
            } else {
                val = chars[i] == chars[j] && flag[i + 1][j - 1];
            }
            let range_len = j - i + 1;
            if val && range_len > max {
                max = range_len;
                max_start = i;
            }
            flag[i][j] = val;
        }
    }
    String::from_utf8(chars[max_start..max_start + max].to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::dp::longest_palindromic_substring::solution;

    #[test]
    fn check() {
        assert_eq!(solution("123".to_string()), "1".to_string());
        assert_eq!(solution("12343".to_string()), "343".to_string());
        assert_eq!(solution("1234321".to_string()), "1234321".to_string());
    }
}
