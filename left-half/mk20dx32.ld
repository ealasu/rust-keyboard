_sflashdata = LOADADDR(.data);

MEMORY {
    FLASH (rx) : ORIGIN = 4096, LENGTH = 256K
    RAM (rwx) : ORIGIN = 0x20000000 - 8K / 2, LENGTH = 8K
}

SECTIONS {
    . = 0;
    .text : {
        . = 0; KEEP(*(.vectors))
        *(.startup*)
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.fastrun*)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH

    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    _estack = ORIGIN(RAM) + LENGTH(RAM); /* stack pointer start */
}
