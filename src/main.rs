use serial_io::{bindings::c_getch, Access, SerialPort};
use std::io::{Read, Result, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    ctrlc::set_handler(|| {}).expect("无法忽略SIGINT信号");

    let mut args = std::env::args();
    let com = std::env::args()
        .nth(1)
        .expect(format!("Usage: {} com<index>", args.nth(0).unwrap()).as_str());

    let stop = Arc::new(Mutex::new(false));

    let mut sp = SerialPort::try_new(com.as_str(), Access::All).unwrap();
    sp.init().unwrap();

    let sp = Arc::new(Mutex::new(sp));

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
                    print!("{}", String::from_utf8(buf[0..size].to_vec()).unwrap());
                    std::io::stdout().flush().unwrap();
                } else {
                    thread::sleep(Duration::from_micros(100));
                }
            }
        })
    };

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
