#![no_std] //Disable standard library
#![no_main] //We do not have access to crt0, so we setup the enviroinment ourselves
#![feature(custom_test_frameworks)]
#![test_runner(o2_h2::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
use core::panic::PanicInfo;


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    o2_h2::test_panic_handler(info)
}


#[no_mangle] //Disable name mangling so that the function name is really _start
pub extern "C" fn _start() -> ! { //We have to use C calling conventions instead of Rust calling conventions
    //. Also the function is diverging (CANNOT RETURN), thus the ! return type

    o2_h2::interrupts::init(); //Load exceptions support
    x86_64::instructions::interrupts::int3();     // invoke a breakpoint exception


    println!("Hello World{}\n TEST:{}\n IT WORKS!", "!", 1);
    //panic!("THE OS IS STILL WIP");

    #[cfg(test)]
    test_main();

    loop {}
}