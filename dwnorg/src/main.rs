mod error;
mod ext;

use crate::ext::PathBuf;
use clap::Parser;

// #[derive(Debug)]
// struct Group {
//     name: String,
//     exts: Vec<String>,
// }

#[derive(Debug, Parser)]
struct Args {
    /// whether to ignore dot files or not
    #[arg(short = 'I', long, default_value_t = true)]
    ignore_hidden: bool,

    /// whether to follow gitignore file or not
    #[arg(short, long)]
    gitignore: bool,

    /// prints what changes would be made
    #[arg(short, long)]
    dry_run: bool,

    /// this can be a path to any gitignore file or a file
    /// that follows the format and rules of gitignore file.
    ///
    /// this option will overwrite the `gitignore` flag
    #[arg(short = 'G', long, name = "ignore_path")]
    gitignore_path: Option<::std::path::PathBuf>,

    #[arg(
        short = 'D',
        long,
        name = "dwn_path",
        default_value_t = PathBuf::try_from(dirs::download_dir()).unwrap()
    )]
    dwn_folder_path: PathBuf,
}

fn main() {
    // #[deny(unused)]
    // #[cfg_attr(debug_assertions, allow(unused))]
    let args = Args::parse();
    println!("{:#?}", args);
}
