#![feature(const_let)]

static mut STDERR_BUFFER_SPACE: u8 = 0;

pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
//~^ ERROR references in statics may only refer to immutable values

fn main() {}
