#![no_std]
#![no_main]

use core::panic::PanicInfo;
use miglix::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("should panic test did not panic.");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_println!("should panic test start...");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}