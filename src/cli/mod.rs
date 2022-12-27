use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "ydd_180",
    version = "0.1.1",
    about = "a windows serial port tools",
    long_about = "ctrl+B or ctrl+B to exit the connection"
)]
pub struct Cli {
    /// enable utf-8 encoding mode, default: GB2312
    #[arg(long)]
    pub utf8: bool,

    /// serial name like `com10`
    #[arg(required = true)]
    pub com_name: String,
}
