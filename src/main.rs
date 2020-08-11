#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use rust_os::println;
use core::panic::PanicInfo;
extern crate rlibc;


//cargo build --target thumbv7em-none-eabihf

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "1");
    println!("Hello World{}", "2");

    rust_os::init();

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
