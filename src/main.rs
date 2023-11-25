#![no_std]
#![no_main]

mod arch;

#[no_mangle]
fn efi_main() {
    // println!("Hello, world!");
    loop {}
}

#[no_mangle]
fn _relocate() {
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
