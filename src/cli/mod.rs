use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "ydd_180",
    version = "0.1.3",
    about = "a windows serial port tools",
    long_about = "ctrl+B or ctrl+B to exit the connection"
)]
pub struct Cli {
    /// enable GB2312 encoding mode, default: UTF-8
    #[arg(long)]
    pub gb2312: bool,

    // #[arg(short, long, value_name = "list", action)]
    // pub list_port: bool,
    /// serial port name like `com10`
    #[arg(required = false, value_name = "serial port")]
    pub port_name: String,
}
