use clap::Parser;
use serial_io::{bindings::c_getch, cli};
use std::io::ErrorKind::TimedOut;
use std::io::{Read, Result, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use textcode::gb2312;

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
    let port_name = args.port_name;
    let use_gb2312 = args.gb2312;
    // let list_port = args.list_port;

    // if list_port == true {
    //     let ports = serialport::available_ports().expect("No ports found!");
    //     for p in ports {
    //         println!("{}({:?})", p.port_name, p.port_type);
    //     }
    //     return Ok(());
    // }

    // 初始化
    let display = if use_gb2312 == true {
        |buf: &[u8]| {
            print!("{}", gb2312::decode_to_string(buf));
            std::io::stdout().flush().unwrap();
        }
    } else {
        |buf: &[u8]| {
            print!("{}", String::from_utf8(buf.to_vec()).unwrap());
            std::io::stdout().flush().unwrap();
        }
    };
    let stop = Arc::new(Mutex::new(false));

    let port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let port = Arc::new(Mutex::new(port));

    // 接受线程
    let recive_thread = {
        let stop = stop.clone();
        let port = port.clone();
        thread::spawn(move || {
            let mut serial_buf: Vec<u8> = vec![0; 32];
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
                    thread::sleep(Duration::from_micros(10));
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
