use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKERSCRIPT_FILENAME: &str = "link-rk3566-bt0.ld";

// NOTE: The base address when run from the mask ROM is system SRAM + 0x1000.
const LINKERSCRIPT: &[u8] = b"
OUTPUT_ARCH(aarch64)
ENTRY(start)
MEMORY {
    XRAM : ORIGIN = 0x00000000, LENGTH = 64k
    SRAM : ORIGIN = 0xfdcc1000, LENGTH = 63k
}
SECTIONS {
    .head : {
        *(.head.text)
    } > SRAM
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(8);
    } > SRAM
    .bss : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > SRAM

    # https://docs.rust-embedded.org/embedonomicon/main.html
    .rodata : {
        *(.rodata .rodata.*);
    } > SRAM #FLASH
    .data : {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > SRAM
    _sidata = LOADADDR(.data);

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug_*)
        *(.comment*)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKERSCRIPT_FILENAME))
        .unwrap()
        .write_all(LINKERSCRIPT)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
