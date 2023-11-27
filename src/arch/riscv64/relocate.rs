use core::{ffi::c_void, hint::spin_loop, ptr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dyn {
    pub d_tag: i64,
    pub(super) d_un: u64,
}

impl Dyn {
    pub fn d_val(&self) -> u64 {
        self.d_un
    }

    pub fn d_ptr(&self) -> u64 {
        self.d_un
    }
}

#[no_mangle]
unsafe extern "C" fn efi_relocate(ldbase: u64, elf_dyn: *mut c_void) -> usize {
    let elf_dyn = elf_dyn as *mut Dyn;
    return do_relocate(ldbase, elf_dyn).0 as usize;
}

unsafe fn do_relocate(ldbase: u64, elf_dyn: *mut Dyn) -> uefi::Status {
    let mut relsz = 0;
    let mut relent = 0;
    let mut rel: *mut elf::relocation::Elf64_Rela = ptr::null_mut();

    let mut item = elf_dyn;

    while (*item).d_tag != elf::abi::DT_NULL {
        match (*item).d_tag {
            elf::abi::DT_RELA => {
                rel = (*item).d_ptr() as *mut elf::relocation::Elf64_Rela;
            }
            elf::abi::DT_RELASZ => {
                relsz = (*item).d_val();
            }
            elf::abi::DT_RELAENT => {
                relent = (*item).d_val();
            }
            _ => {}
        }
        item = (item as usize + core::mem::size_of::<Dyn>()) as *mut Dyn;
    }

    if rel.is_null() && (relent == 0) {
        return uefi::Status::SUCCESS;
    }

    if rel.is_null() || relent == 0 {
        return uefi::Status::LOAD_ERROR;
    }

    while relsz > 0 {
        match ((*rel).r_info & 0xFF) as u32 {
            elf::abi::R_RISCV_RELATIVE => {
                let addr = ldbase + (*rel).r_offset as u64;
                let sym_addr = ldbase + (*rel).r_addend as u64;
                let addr_ptr = addr as *mut u64;
                *addr_ptr = sym_addr;
            }
            _ => {
                /* Panic */
                loop {
                    spin_loop();
                }
            }
        }
        rel = (rel as usize + relent as usize) as *mut elf::relocation::Elf64_Rela;
        relsz -= relent;
    }

    return uefi::Status::SUCCESS;
}
