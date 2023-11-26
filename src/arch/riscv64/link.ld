/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * U-Boot riscv64 EFI linker script
 *
 * SPDX-License-Identifier:     BSD-2-Clause
 *
 * Modified from arch/arm/lib/elf_aarch64_efi.lds
 */
/**
 * Sourced from
 * https://source.denx.de/u-boot/u-boot/-/blob/52ba373b7825e9feab8357065155cf43dfe2f4ff/arch/riscv/lib/elf_riscv64_efi.lds
 */

OUTPUT_FORMAT("elf64-littleriscv", "elf64-littleriscv", "elf64-littleriscv")
OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS
{
        .text 0x0 : SUBALIGN(16) {
                _text = .;
                *(.text.head)
                *(.text)
                *(.text.*)
                *(.gnu.linkonce.t.*)
                *(.srodata)
                *(.rodata*)
                *(.got)
                *(.got.*)
                . = ALIGN(16);
        }
        
        _etext = .;
        . = ALIGN(16);
        _text_size = . - _text;
        .dynamic  : { *(.dynamic) }
        .data : {
                _data = .;
                *(.sdata)
                *(.sdata.*)
                *(.data)
                *(.data1)
                *(.data.*)
                *(.got.plt)
                *(.got.plt.*)
                

                /*
                 * The EFI loader doesn't seem to like a .bss section, so we
                 * stick it all into .data:
                 */
                . = ALIGN(16);
                _bss = .;
                *(.sbss)
                *(.scommon)
                *(.dynbss)
                *(.bss)
                 
                *(.bss.*)
                *(COMMON)
                . = ALIGN(16);
                _bss_end = .;
        }
        
        .rela (INFO) : {
            *(.rela .rela*)
        }

        
        /*
         * Put _edata here so it all gets loaded by U-Boot.
         * The script originally had this equal to _bss_end, but
         * our .rela.dyn wasn't getting loaded into memory so we
         * couldn't do any relocations.
         *
         * Here be dragons.  Do we need .dynsym/.dynstr too?
         */
        _edata = .;
        _data_size = . - _etext;

        . = ALIGN(4096);
        .dynsym   : { *(.dynsym) }
        . = ALIGN(4096);
        .dynstr   : { *(.dynstr) }
        . = ALIGN(4096);
        .note.gnu.build-id : { *(.note.gnu.build-id) }
        /DISCARD/ : {
                
                *(.eh_frame)
                *(.note.GNU-stack)
        }
        .comment 0 : { *(.comment) }
}
