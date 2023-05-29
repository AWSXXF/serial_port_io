use clap::{CommandFactory, Parser};
use serial_io::{bindings::c_getch, cli};
use std::io::ErrorKind::TimedOut;
use std::io::{Read, Result, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use textcode::{gb2312, utf8};

fn main() -> Result<()> {
    // if cfg!(target_os = "windows") {
    //     std::process::Command::new("chcp")
    //         .env("PATH", std::env!("PATH"))
    //         .arg("65001")
    //         .output()
    //         .unwrap();
    // }

    // 命令行参数
    ctrlc::set_handler(|| {}).expect("无法忽略SIGINT信号");
    let args = cli::Cli::parse();

    let list = args.list_port;
    let rate = args.baud_rate;
    let port_name = args.port_name;
    let use_gb2312 = args.gb2312;

    if list == true {
        let available_ports = serialport::available_ports().expect("读取可用串口失败");
        for port in available_ports.iter() {
            println!("{}\t{:?}", port.port_name, port.port_type);
        }
        return Ok(());
    }

    let port_name = match port_name {
        Some(name) => name,
        None => {
            cli::Cli::command().print_help().unwrap();
            return Ok(());
        }
    };

    // 初始化
    let display = if use_gb2312 == true {
        |buf: &[u8]| {
            print!("{}", gb2312::decode_to_string(buf));
            std::io::stdout().flush().unwrap();
        }
    } else {
        |buf: &[u8]| {
            print!("{}", utf8::decode_to_string(buf));
            std::io::stdout().flush().unwrap();
        }
    };
    let stop = Arc::new(Mutex::new(false));

    let port = serialport::new(&port_name, rate)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let port = Arc::new(Mutex::new(port));

    // 提示信息
    println!(
        "PORT open: name({}) bound rate({}) encode({})",
        port_name,
        rate,
        if use_gb2312 { "GB2312" } else { "UTF-8" }
    );
    println!("ctrl+A or ctrl+B to exit the connection");

    // 接受线程
    let recive_thread = {
        let stop = stop.clone();
        let port = port.clone();
        thread::spawn(move || {
            let mut serial_buf: Vec<u8> = vec![0; 1024];
            let mut size = 0;
            let _ = size; // just make compiler happy

            loop {
                if *stop.lock().unwrap() == true {
                    return;
                }

                {
                    let mut port = port.lock().unwrap();

                    size = match port.read(serial_buf.as_mut_slice()) {
                        Ok(ret) => ret,
                        Err(err) if err.kind() == TimedOut => 0,
                        Err(err) => panic!("serial read error: {}", err.to_string()),
                    };
                }

                if size != 0 {
                    display(&serial_buf[0..size]);
                } else {
                    thread::sleep(Duration::from_micros(1));
                }
            }
        })
    };

    // 发送线程
    let write_thread = {
        let stop = stop.clone();
        let port = port.clone();
        thread::spawn(move || {
            let mut ch = [0u8; 1];
            loop {
                if *stop.lock().unwrap() == true {
                    return;
                }
                ch[0] = c_getch() as u8;
                if ch[0] == 0x01 || ch[0] == 0x02 {
                    *stop.lock().unwrap() = true;
                    break;
                }
                {
                    let mut port = port.lock().unwrap();
                    port.write(&ch).expect("Write failed!");
                }
            }
        })
    };

    write_thread.join().unwrap();
    recive_thread.join().unwrap();

    Ok(())
}
