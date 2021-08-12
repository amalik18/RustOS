#![no_std]
#![no_main]
/*
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
*/

use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[The Test did NOT panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Test Runner function.
/// Not needed since `harness` is disabled for this test.
/*
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests...", tests.len());
    for test in tests {
        test();
        serial_println!("[Test did NOT panic]");
        exit_qemu(QemuExitCode::Failed)
    }
    exit_qemu(QemuExitCode::Success);
}
*/

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// #[test_case]
fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0,1);
}