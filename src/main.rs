use serial_io::SerialPort;
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let mut sp = SerialPort::try_new("com2")?;
    sp.init()?;

    let mut buf: [u8; 255] = [0; 255];
    let mut size = 0;
    let _ = size; // just make compiler happy

    loop {
        size = sp.read(&mut buf)?;
        print!("{}", String::from_utf8(buf[0..size].to_vec()).unwrap());
        std::io::stdout().flush()?;

        sp.write(&buf[0..size])?;

        buf.iter_mut().for_each(|v| {
            *v = 0;
        });
        size = 0;
        let _ = size; // just make compiler happy
    }
    // Ok(())
}
