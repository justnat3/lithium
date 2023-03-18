#![no_std]
#![no_main]

static HELLO: &[u8] = b"I can print any string here";
const GRAY: u8 = 0x7;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize *2) = byte;
            *vga.offset(i as isize *2 + 1) = GRAY;
        }
    }
    loop{}
}

