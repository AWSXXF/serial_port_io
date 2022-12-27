use clap::Parser;
use serial_io::{bindings::c_getch, cli, Access, SerialPort};
use std::io::{Read, Result, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use textcode::gb2312;

fn main() -> Result<()> {
    // 命令行参数
    ctrlc::set_handler(|| {}).expect("无法忽略SIGINT信号");
    let args = cli::Cli::parse();
    let com = args.com_name;
    let use_utf8 = args.utf8;

    // 初始化
    let display = if use_utf8 == true {
        |buf: &[u8]| {
            print!("{}", String::from_utf8(buf.to_vec()).unwrap());
            std::io::stdout().flush().unwrap();
        }
    } else {
        |buf: &[u8]| {
            print!("{}", gb2312::decode_to_string(buf));
            std::io::stdout().flush().unwrap();
        }
    };
    let stop = Arc::new(Mutex::new(false));
    let mut sp = SerialPort::try_new(format!("\\\\.\\{}", com).as_str(), Access::All).unwrap();
    sp.init().unwrap();
    let sp = Arc::new(Mutex::new(sp));

    // 接受线程
    let recive_thread = {
        let stop = stop.clone();
        let sp = sp.clone();
        thread::spawn(move || {
            let mut buf: [u8; 255] = [0; 255];
            let mut size = 0;
            let _ = size; // just make compiler happy

            loop {
                if *stop.lock().unwrap() == true {
                    return;
                }

                {
                    let mut sp = sp.lock().unwrap();
                    size = sp.read(&mut buf).unwrap();
                }
                if size != 0 {
                    display(&buf[0..size]);
                } else {
                    thread::sleep(Duration::from_micros(10));
                }
            }
        })
    };

    // 发送线程
    let write_thread = {
        let stop = stop.clone();
        let sp = sp.clone();
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
                    let mut sp = sp.lock().unwrap();
                    sp.write(&ch).unwrap();
                }
            }
        })
    };

    write_thread.join().unwrap();
    recive_thread.join().unwrap();

    Ok(())
}
