#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(miglix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use miglix::serial_println;

#[no_mangle] // この関数の名前を変えない
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    serial_println!("test println output on basic_boot.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    miglix::test_panic_handler(info)
}