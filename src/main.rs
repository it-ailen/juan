mod subcommand;
mod leetcode;

use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "zouyalong<zyl_work@163.com>")]
struct RootOpts {}

fn main() {
    let opts = RootOpts::parse();
    println!("Hello, world!");
}
