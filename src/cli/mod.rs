use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = "ctrl+A or ctrl+B to exit the connection",
)]

pub struct Cli {
    /// List the available port
    #[arg(short, long)]
    pub list_port: bool,

    /// Setting boud rate, default: 115200
    #[arg(short,long,action=clap::ArgAction::Set, default_value="115200")]
    pub baud_rate: u32,

    /// Show charactor as GB2312 encoding, default: UTF-8
    #[arg(long)]
    pub gb2312: bool,

    /// Serial port name like `com10`
    #[arg(value_name = "serial port")]
    pub port_name: Option<String>,
}
