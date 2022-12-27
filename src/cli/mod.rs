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
    pub gb2312: bool,

    // #[arg(short, long, value_name = "list", action)]
    // pub list_port: bool,
    /// serial port name like `com10`
    #[arg(required = false, value_name = "serial port")]
    pub port_name: String,
}
