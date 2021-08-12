#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

/***
Don't mangle the name of the function
This function is the entry point, since the linke looks for a function named
`_start` by default
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::print_something();
    // vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", som numbers: {}, {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");

    #[cfg(test)]
        test_main();
    loop {}
}

// Panic Handler, function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn main_trivial_assertion() {
    assert_eq!(1, 1);
}
