#![no_std] // don't link the standard library
#![no_main] // disable all Rust entry-point

use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {  // entry point : linker looks for a function called '_start' by default
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}