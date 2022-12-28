use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "ydd_180",
    version = "0.1.4",
    about = "a windows serial port tools",
    long_about = "ctrl+B or ctrl+B to exit the connection"
)]
pub struct Cli {
    /// Show charactor as GB2312 encoding, default: UTF-8
    #[arg(long)]
    pub gb2312: bool,

    /// List the available port
    #[arg(short, long)]
    pub list_port: bool,

    /// Serial port name like `com10`
    #[arg(value_name = "serial port")]
    pub port_name: Option<String>,
}
