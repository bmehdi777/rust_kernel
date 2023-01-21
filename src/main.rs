#![no_std] // don't link the standard library
#![no_main] // disable all Rust entry-point

use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {  // entry point : linker looks for a function called '_start' by default
    println!("Hello world !{}", "!");
    panic!("Ouch");

    loop {}
}
