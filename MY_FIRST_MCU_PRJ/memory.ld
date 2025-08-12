/*See STM32F303 data sheet*/

ENTRY(reset_handler) /*Entry point for the program, reset_handler is defined in startup file*/

/*STM32f303*/
MEMORY {
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 40K
    /*EEPROM (rwx) : ORIGIN = 0x08080000, LENGTH = 4K*/
    /*CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 8K*/
    /*BATTRAM (rwx) : ORIGIN = 0x40024000, LENGTH = 4K*/ /*looks wrong*/
}

/*STM32h563*/

/* MEMORY {
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 2M
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 655360
    BATTRAM (rwx) : ORIGIN = 0x50036400, LENGTH = 4K
} */


_start_of_stack = ORIGIN(RAM) + LENGTH(RAM); /*End of RAM +1*/
_min_stack_size = 0x400; /*Minimum stack size, 1K*/
_min_heap_size = 0x400; /*Minimum heap size, 1K*/



SECTIONS
{
    .text :
    {
        . = ALIGN(4);
        LONG( _start_of_stack) /*Initial stack pointer*/
        KEEP(*(.isr_vector)) /*Startup code*/
        *(.text)
        *(.text*)
        . = ALIGN(4);
    } > FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data : {
        _sidata = LOADADDR(.data); /* Start of data section in LMA(flash)*/
        . = ALIGN(4);
        _sdata = .; /* Start of data section in VMA(ram)*/
        *(.data)
        *(.data*)
        _edata = .; /* End of data section */
        . = ALIGN(4);
    } > RAM AT> FLASH

    .bss : {
        . = ALIGN(4);
        _sbss = .; /* Start of bss section in VMA(ram)*/
        *(.bss)
        *(.bss*)
        _ebss = .; /* End of bss section */
        . = ALIGN(4);
    } > RAM

    .ram_usage_check : {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}



INCLUDE device_STM32F303.x
INCLUDE device_STM32H563.x