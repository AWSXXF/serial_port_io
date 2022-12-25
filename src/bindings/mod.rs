mod c_bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub fn c_getch() -> i32 {
    (unsafe { c_bindings::_getch() }) as i32
}
