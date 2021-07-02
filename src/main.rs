#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;
// use core::fmt::Write;



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
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[derive(Debug, CLone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial Assertion....");
    assert_eq!(1, 1);
    println!("[Ok]");
}
