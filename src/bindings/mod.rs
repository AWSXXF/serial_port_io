mod c_bindings {
    #![warn(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub fn c_getch() -> i32 {
    (unsafe { c_bindings::_getch() }) as i32
}

pub fn c_getwch() -> u16 {
    (unsafe { c_bindings::_getwch() }) as u16
}
