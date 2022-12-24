use std::ffi::c_void;
use std::io::Result;
use std::iter;
use std::ops::BitOr;

pub use windows::core::PCWSTR;
pub use windows::Win32::Devices::Communication::COMMTIMEOUTS;
pub use windows::Win32::Devices::Communication::DCB;
pub use windows::Win32::Devices::Communication::PURGE_COMM_FLAGS;
pub use windows::Win32::Foundation::CloseHandle;
pub use windows::Win32::Foundation::HANDLE;
pub use windows::Win32::Storage::FileSystem::FILE_ACCESS_FLAGS;
pub use windows::Win32::Storage::FileSystem::FILE_GENERIC_READ;
pub use windows::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;

use windows::Win32::{
    Devices::Communication::{
        PurgeComm, SetCommState, SetCommTimeouts, SetupComm, NOPARITY, ONESTOPBIT, PURGE_RXCLEAR,
        PURGE_TXCLEAR,
    },
    Storage::FileSystem::{CreateFile2, ReadFile, WriteFile, FILE_SHARE_NONE, OPEN_EXISTING},
};

pub fn purge_comm(com: HANDLE) -> Result<()> {
    unsafe {
        PurgeComm(
            com,
            PURGE_COMM_FLAGS(0)
                .bitor(PURGE_RXCLEAR)
                .bitor(PURGE_TXCLEAR),
        )
        .ok()?;
    }
    Ok(())
}

pub fn set_comm_rate(com: HANDLE, bound_rate: u32) -> Result<()> {
    unsafe {
        SetCommState(com, &{
            let mut dcb = DCB::default();
            dcb.BaudRate = bound_rate;
            dcb.ByteSize = 8;
            dcb.Parity = NOPARITY;
            dcb.StopBits = ONESTOPBIT;
            dcb
        })
        .ok()?;
    }
    Ok(())
}

pub fn set_comm_timeouts(com: HANDLE, timeouts: COMMTIMEOUTS) -> Result<()> {
    unsafe {
        SetCommTimeouts(com, &timeouts).ok()?;
    }
    Ok(())
}

pub fn setup_comm(com: HANDLE, in_queue_size: u32, out_queue_size: u32) -> Result<()> {
    unsafe {
        SetupComm(com, in_queue_size, out_queue_size).ok()?;
    }
    Ok(())
}

pub fn create_comm(com_name: &str, file_flag: FILE_ACCESS_FLAGS) -> Result<HANDLE> {
    let com_name: Vec<u16> = String::from(com_name)
        .encode_utf16()
        .chain(iter::once(0))
        .collect();

    let com = unsafe {
        CreateFile2(
            PCWSTR(com_name.as_ptr()),
            file_flag,
            FILE_SHARE_NONE,
            OPEN_EXISTING,
            None,
        )?
    };
    Ok(com)
}

pub fn read_comm(com: HANDLE, buffer: &[u8]) -> Result<usize> {
    let buffer_size: usize = buffer.len();
    let size: usize = 0;
    unsafe {
        ReadFile(
            com,
            Some(buffer.as_ptr() as *mut c_void),
            buffer_size as u32,
            Some(&size as *const usize as *mut u32),
            None,
        )
        .ok()?;
    }
    Ok(size)
}

pub fn write_comm(com: HANDLE, buffer: &[u8], size: usize) -> Result<usize> {
    unsafe {
        WriteFile(
            com,
            Some(buffer.as_ptr() as *mut c_void),
            size as u32,
            Some(&size as *const usize as *mut u32),
            None,
        )
        .ok()?;
    }
    Ok(size)
}
