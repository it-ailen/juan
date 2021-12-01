use super::dp::Problem;

struct Solution(usize);

impl Problem<String, String> for Solution {
    fn solution(path: String) -> String {
        let bytes = path.as_bytes();
        let mut stack: Vec<&str> = Vec::new();
        let mut cursor_slow = 0usize;
        let mut cursor_fast: usize = 0usize;
        for i in 1..path.len() {
            cursor_fast = i;
            let c = bytes[i] as char;
            if c == '/' {
                if cursor_fast - cursor_slow == 1 {
                    // cursor_slow 和 cursor_fast 都为 /，对应 // 的情况，跳过
                } else {
                    let sub = &path[cursor_slow + 1..cursor_fast];
                    match sub {
                        "." => {}
                        ".." => {stack.pop();},
                        _ => stack.push(sub), 
                    }
                }
                cursor_slow = cursor_fast;
            }
        }

        if cursor_fast > cursor_slow {
            let sub = &path[cursor_slow + 1..=cursor_fast];
            match sub {
                "." => {}
                ".." => {stack.pop();},
                _ => stack.push(sub), 
            } 
        }
        let mut result = String::new();
        if stack.len() == 0 {
            result.push('/');
        } else {
            for p in stack {
                result.push('/');
                result.push_str(p);
            }
        }
        result
    }

    fn id() -> usize {
        71
    }

    fn name() -> &'static str {
        "简化路径"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/simplify-path/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{dp::Problem, simplify_path71::Solution};

    #[test]
    fn test() {
        assert_eq!(
            Solution::solution("/".to_string()),
            "/".to_string()
        );
        assert_eq!(
            Solution::solution("/home/".to_string()),
            "/home".to_string()
        );
        assert_eq!(Solution::solution("/../".to_string()), "/".to_string());
        assert_eq!(
            Solution::solution("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
        assert_eq!(
            Solution::solution("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
        assert_eq!(
            Solution::solution("/a//b////c/d//././/..".to_string()),
            "/a/b/c".to_string()
        );
    }
}
