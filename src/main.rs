#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

	println!("Hello World{}", "!");
	panic!("Some panic message");
    loop {}
}