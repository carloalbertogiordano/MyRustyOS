#![no_std] //Disable standard library
#![no_main] //We do not have access to crt0, so we setup the enviroinment ourselves

mod vga_buffer;


use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!\n");
    println!("{}", info);
    loop {}
}


#[no_mangle] //Disable name mangling so that the function name is really _start
pub extern "C" fn _start() -> ! { //We have to use C calling conventions instead of Rust calling conventions
    //. Also the function is diverging (CANNOT RETURN), thus the ! return type

    println!("Hello World{}\n TEST:{}\n IT WORKS!", "!", 1);
    //panic!("THE OS IS STILL WIP");

    loop {}
}