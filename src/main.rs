#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(miglix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use miglix::println;

/// この関数はパニック時に呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	miglix::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

	println!("Hello World{}", "!");

	miglix::init();

	// issue breakpoint handler exception
	x86_64::instructions::interrupts::int3(); 

	#[cfg(test)]
	test_main();
	
	println!("It did not crash.");
	loop{}
}