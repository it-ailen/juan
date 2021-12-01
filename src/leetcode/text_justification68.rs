use super::dp::Problem;


struct Solution(usize);

impl Problem<(Vec<String>, i32), Vec<String>> for Solution {
    fn solution(i: (Vec<String>, i32)) -> Vec<String> {
        // let words = i.0;
        // let max_width = i.1;
        // if words.len() == 0 {
        //     return words.clone();
        // }
        // let mut line_start = 0usize;
        // let mut line_sum = words[0].len();
        // for (idx, w) in words.iter().enumerate().skip(1) {
        //     let cur_len = w.len();
        //     if line_sum + cur_len + 1 >= max_width as usize {
        //         let padding = max_width as usize - line_sum;
        //         let gaps = (idx - line_start + 1) - 1;
        //         let right 
        //         let left_blank_cnt = 
        //     }
        // }
        return Vec::new();
    }

    fn id() -> usize {
        68
    }

    fn name() -> &'static str {
        "文本左右对齐"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/text-justification/"
    }
}