__rpi_load_addr = 0x80000;

ENTRY(__rpi_load_addr)

/* HEADERS */
PHDRS
{
    segment_rx PT_LOAD FLAGS(5);
    segment_rw PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = __rpi_load_addr;

    __boot_core_stack_ende = .;

    __rx_start = .;
    .text :
    {
        KEEP(*(.text._start))
        *(.text*)
    } :segment_rx

    .rodata : ALIGN(8) { *(.rodata*) } :segment_rx
    .got    : ALIGN(8) { *(.got)     } :segment_rx

    . = ALIGN(64K);
    __rx_ende = .;

    __rw_start = .;
    .data : { *(.data*) } :segment_rw

    .bss : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_ende = .;
    } :NONE

    . = ALIGN(64K);
    __rw_ende = .;

    __boot_core_stack_guard_page_start = .;
    . += 64K;
    __boot_core_stack_guard_page_ende = .;

    __boot_core_stack_start = .;
    . += 512K;
    __boot_core_stack_ende = .;
}
