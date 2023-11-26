#![no_std]
#![no_main]
#![feature(fmt_internals)]

use core::{
    ffi::c_void,
    fmt::{Formatter, Write},
};
extern crate alloc;

use alloc::string::String;
use log::info;
use uefi::{
    table::{Boot, SystemTable},
    CStr16, Handle,
};

mod arch;

extern "C" {
    fn _start() -> !;
}

#[no_mangle]
unsafe extern "efiapi" fn efi_main(
    handle_ptr: *mut c_void,
    system_table_ptr: *mut c_void,
) -> usize {
    rs_efi_main(handle_ptr, system_table_ptr)
        .map(|_| uefi::Status::SUCCESS.0 as usize)
        .unwrap_or_else(|e| e.0 as usize)
}

fn rs_efi_main(
    handle_ptr: *mut c_void,
    system_table_ptr: *mut c_void,
) -> Result<(), uefi::prelude::Status> {
    let image_handle =
        unsafe { Handle::from_ptr(handle_ptr).ok_or(uefi::Status::INVALID_PARAMETER)? };
    let mut system_table: SystemTable<uefi::table::Boot> =
        unsafe { SystemTable::from_ptr(system_table_ptr).ok_or(uefi::Status::INVALID_PARAMETER)? };
    unsafe { system_table.boot_services().set_image_handle(image_handle) };

    uefi_services::init(&mut system_table).map_err(|e| e.status())?;

    let mut buf = [0u16; 32];
    system_table.stdout().write_str("123455\n");
    let x = CStr16::from_str_with_buf("DragonBoot Starting...\n", &mut buf).unwrap();
    system_table
        .stdout()
        .output_string(x)
        .map_err(|e| e.status())?;

    let x = String::from("AAAAAAAHello, world!\n");
    system_table.stdout().write_str(x.as_str());

    let args = core::format_args!("hgfhgfhfHello, world!\n");
    // 这里println的FormatWriter里面的&'a dyn (Write +'a)貌似不能在重定位之前访问，不然的话会出现错误：
    // ```
    // Found EFI removable media binary efi/boot/bootriscv64.efi
    // 20336 bytes read in 4 ms (4.8 MiB/s)
    // Booting /efi\boot\bootriscv64.efi
    // 123455
    // DragonBoot Starting...
    // AAAAAAAHello, world!
    // Unhandled exception: Illegal instruction
    // EPC: 00000000000012b0 RA: 000000009deee240 TVAL: 0000000000000000
    // EPC: ffffffffe0abf2b0 RA: 000000007e9ac240 reloc adjusted
    // Code: 0000 0000 0000 0000 0000 0000 0000 0000 (0000)
    // UEFI image [0x000000009deec000:0x000000009def0f6f] '/efi\boot\bootriscv64.efi
    // ```
    //
    // Fault的PC和值都是错误的，因此猜想动态分发这块的代码不是PIC的，需要重定位，因此不能在重定位之前用println

    loop {}
    // 执行下面这行会出错，就是上面注释说的那个错误
    system_table.stdout().write_fmt(args);
    // system_table.stdout().write_str(args);

    loop {}
    return Ok(());
}

#[no_mangle]
fn _relocate() {
    loop {}
}

pub fn x() {}
