#![no_std] //Disable standard library
#![no_main] //We do not have access to crt0, so we setup the enviroinment ourselves
#![feature(custom_test_frameworks)]
#![test_runner(o2_h2::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    o2_h2::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    o2_h2::test_panic_handler(info)
}

//new entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {

    use o2_h2::memory;
    use x86_64::{structures::paging::Page, VirtAddr};
    use o2_h2::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");
    o2_h2::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    o2_h2::hlt_loop();
}

/* //Old entry point
#[no_mangle] //Disable name mangling so that the function name is really _start
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! { //We have to use C calling conventions instead of Rust calling conventions
    //. Also the function is diverging (CANNOT RETURN), thus the ! return type

    o2_h2::init(); //Load exceptions support
    x86_64::instructions::interrupts::int3(); // invoke a breakpoint exception

    // Note: The actual address might be different for you. Use the address that
    // your page fault handler reports.
    let ptr = 0x2031b2 as *mut u32;

    // read from a code page
    unsafe { let x = *ptr; }
    println!("read worked");

    // write to a code page
    unsafe { *ptr = 42; }
    println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    o2_h2::hlt_loop();  
}
*/