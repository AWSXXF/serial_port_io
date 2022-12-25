use crate::win_com::*;

pub use crate::win_com::Access;

use std::io::{Result, Write};
pub struct SerialPort {
    pub name: String,
    com: HANDLE,
    timeouts: COMMTIMEOUTS,
    rate: u32,
    dw_in_queue: u32,
    dw_out_queue: u32,
}

impl SerialPort {
    pub fn try_new(name: &str, access: Access) -> Result<Self> {
        Ok(Self {
            name: String::from(name),
            com: create_comm(name, access)?,
            timeouts: COMMTIMEOUTS {
                ReadIntervalTimeout: 1,
                ReadTotalTimeoutMultiplier: 1,
                ReadTotalTimeoutConstant: 1,
                WriteTotalTimeoutMultiplier: 1,
                WriteTotalTimeoutConstant: 1,
            },
            rate: 115200,
            dw_in_queue: 1024,
            dw_out_queue: 1024,
        })
    }

    pub fn init(&mut self) -> Result<&mut Self> {
        setup_comm(self.com, self.dw_in_queue, self.dw_out_queue)?;
        set_comm_timeouts(self.com, self.timeouts)?;
        self.set_bound_rate(115200)?.flush()?;
        Ok(self)
    }

    pub fn set_read_timeout(&mut self, time: u32) -> Result<&mut Self> {
        self.timeouts.ReadTotalTimeoutConstant = time;
        set_comm_timeouts(self.com, self.timeouts)?;
        Ok(self)
    }

    pub fn set_write_timeout(&mut self, time: u32) -> Result<&mut Self> {
        self.timeouts.WriteTotalTimeoutConstant = time;
        set_comm_timeouts(self.com, self.timeouts)?;
        Ok(self)
    }

    pub fn set_bound_rate(&mut self, rate: u32) -> Result<&mut Self> {
        self.rate = rate;
        set_comm_rate(self.com, rate)?;
        Ok(self)
    }
}

impl std::io::Read for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        read_comm(self.com, buf)
    }
}

impl std::io::Write for SerialPort {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut size: usize = 0;
        while size < buf.len() {
            let left_buf: &[u8] = &buf.clone()[size..buf.len()];
            size = write_comm(self.com, left_buf, buf.len() - size)?;
        }
        Ok(size)
    }

    fn flush(&mut self) -> Result<()> {
        purge_comm(self.com)
    }
}
