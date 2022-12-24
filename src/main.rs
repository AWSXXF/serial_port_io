use std::ffi::{c_void, OsStr};
use std::iter;
use std::ops::BitOr;
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::{
    Devices::Communication::{
        PurgeComm, SetCommState, SetCommTimeouts, SetupComm, COMMTIMEOUTS, DCB, NOPARITY,
        ONESTOPBIT, PURGE_COMM_FLAGS, PURGE_RXCLEAR, PURGE_TXCLEAR,
    },
    Foundation::CloseHandle,
    Storage::FileSystem::{
        CreateFile2, ReadFile, WriteFile, FILE_ACCESS_FLAGS, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
        FILE_SHARE_NONE, OPEN_EXISTING,
    },
};

fn main() -> std::io::Result<()> {
    const COM_QUEUE: u32 = 1024;
    // PCWSTR为 utf16编码，0结尾的C字符串
    // 将字符串切片，用OsStr包装，然后才能使用转换接口
    // 然后进行拓展编码，utf8->utf16
    // 再结尾补0，C/C++语言字符串要求
    // 转成VEC，就是数组
    let com_name: Vec<u16> = OsStr::new("com2")
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    // 另一种方法
    let _: Vec<u16> = String::from("HELLO")
        .encode_utf16()
        .chain(iter::once(0))
        .collect();

    // 转成指针，即可被PCWSTR
    let com = unsafe {
        let com = CreateFile2(
            PCWSTR(com_name.as_ptr()),
            FILE_ACCESS_FLAGS::default()
                .bitor(FILE_GENERIC_WRITE)
                .bitor(FILE_GENERIC_READ),
            FILE_SHARE_NONE,
            OPEN_EXISTING,
            None,
        )
        .expect("无法打开串口");

        SetupComm(com, COM_QUEUE, COM_QUEUE);

        SetCommTimeouts(
            com,
            &(COMMTIMEOUTS {
                ReadIntervalTimeout: u32::MAX,
                ReadTotalTimeoutMultiplier: 3,
                ReadTotalTimeoutConstant: 3,
                WriteTotalTimeoutMultiplier: 1,
                WriteTotalTimeoutConstant: 1,
            }),
        )
        .expect("配置串口超时时间失败");

        SetCommState(com, &{
            let mut dcb = DCB::default();
            dcb.BaudRate = 115200;
            dcb.ByteSize = 8;
            dcb.Parity = NOPARITY;
            dcb.StopBits = ONESTOPBIT;
            dcb
        })
        .expect("配置串口失败");

        PurgeComm(
            com,
            PURGE_COMM_FLAGS(0)
                .bitor(PURGE_RXCLEAR)
                .bitor(PURGE_TXCLEAR),
        )
        .expect("清空串口缓冲区失败");
        com
    };

    const BUFFER_SIZE: usize = 255;

    let mut v: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let len: u32 = 0;

    loop {
        v.iter_mut().for_each(|i| *i = 0);
        unsafe {
            ReadFile(
                com,
                Some(&v as *const _ as *mut c_void),
                v.len() as u32,
                Some(&len as *const u32 as *mut u32),
                None,
            )
            .expect("读取串口错误");
            if len != 0 {
                println!("recive: {}", String::from_utf8(Vec::from(v)).unwrap());
                WriteFile(
                    com,
                    Some(v.as_ptr() as *mut c_void),
                    1,
                    Some(&len as *const u32 as *mut u32),
                    None,
                )
                .expect("写串口错误");
            } else {
                std::thread::sleep(std::time::Duration::from_micros(100));
            }
        }
    }

    unsafe {
        CloseHandle(com);
    }

    Ok(())
    // println!("Hello, world!");
}
