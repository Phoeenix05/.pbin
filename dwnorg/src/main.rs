mod error;

use clap::Parser;
use dwnorg_proc::group;

#[derive(Debug, Parser)]
struct Args {}

group!(archives);

fn main() {
    let args = Args::parse();
}
