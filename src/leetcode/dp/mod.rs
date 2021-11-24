mod longest_palindromic_substring;
mod longsubstringwrc;

use clap::Parser;

// #[derive(Parser)]
// pub struct DP {
//     #[clap()]
//     case: Case,
// }

// enum Case {
//     Case5(),
// }

pub trait Problem<INPUT, OUTPUT> {
    fn solution(i: INPUT) -> OUTPUT;
    fn id() -> usize;
    fn name() -> &'static str;
    fn url() -> &'static str;
}
