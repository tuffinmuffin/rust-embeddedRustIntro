

//define vector table

static VECTOR_TABLE: [Option<unsafe fn()>; 99] = [
    None,                // Initial stack pointer
    Some(reset_handler), // Reset handler
    Some(nmi_handler),                // NMI handler
    Some(hardfault_hander),                // Hard fault handler
    Some(mem_manage_handler),                // Memory management fault handler
    Some(busfault_handler),                // Bus fault handler
    Some(usagefault_handler),                // Usage fault handler
    None,                // Reserved
    None,                // Reserved
    None,                // Reserved
    None,                // Reserved
    Some(default_handler),                // SVCall handler
    Some(default_handler),                // PendSV handler
    Some(default_handler),                // SysTick handler
    Some(default_handler),                // 0 - Window watchdog
    Some(default_handler),                // 1 - PVD through EXTI line detection
    Some(default_handler),                // 2 - Tamper and time stamp
    Some(default_handler),                // 3 - RTC wakeup through EXTI line
    Some(default_handler),                // 4 - Flash
    Some(default_handler),                // 5 - RCC
    Some(default_handler),                // 6 - EXTI line 0
    Some(default_handler),                // 7 - EXTI line 1
    Some(default_handler),                // 8 - EXTI line 2
    Some(default_handler),                // 9 - EXTI line 3
    Some(default_handler),                // 10 - EXTI line 4
    Some(default_handler),                // 11 - DMA1 channel 1
    Some(default_handler),                // 12 - DMA1 channel 2
    Some(default_handler),                // 13 - DMA1 channel 3
    Some(default_handler),                // 14 - DMA1 channel 4
    Some(default_handler),                // 15 - DMA1 channel 5
    Some(default_handler),                // 16 - DMA1 channel 6
    Some(default_handler),                // 17 - DMA1 channel 7
    Some(default_handler),                // 18 - ADC1, ADC2
    Some(default_handler),                // 19 - USB high priority
    Some(default_handler),                // 20 - USB low priority
    Some(default_handler),                // 21 - CAN1 RX1
    Some(default_handler),                // 22 - CAN1 SCE
    Some(default_handler),                // 23 - EXTI line 9 to 5
    Some(default_handler),                // 24 - TIM1 break
    Some(default_handler),                // 25 - TIM1 update
    Some(default_handler),                // 26 - TIM1 trigger and commutation
    Some(default_handler),                // 27 - TIM1 capture compare
    Some(default_handler),                // 28 - TIM2
    Some(default_handler),                // 29 - TIM3
    Some(default_handler),                // 30 - TIM4
    Some(default_handler),                // 31 - I2C1 event
    Some(default_handler),                // 32 - I2C1 error
    Some(default_handler),                // 33 - I2C2 event
    Some(default_handler),                // 34 - I2C2 error
    Some(default_handler),                // 35 - SPI1
    Some(default_handler),                // 36 - SPI2
    Some(default_handler),                // 37 - USART1
    Some(default_handler),                // 38 - USART2
    Some(default_handler),                // 39 - USART3
    Some(default_handler),                // 40 - EXTI line 15 to 10
    Some(default_handler),                // 41 - RTC alarm through
    Some(default_handler),                // 42 - USB wakeup from suspend
    Some(default_handler),                // 43 - TIM8 break
    Some(default_handler),                // 44 - TIM8 update
    Some(default_handler),                // 45 - TIM8 trigger and commutation
    Some(default_handler),                // 46 - TIM8 capture compare
    Some(default_handler),                // 47 - ADC3
    Some(default_handler),                // 48 - FMC
    None,                // 49 - RSVED
    None,                // 50 - RSVED
    Some(default_handler),                // 51 - SPI3
    Some(default_handler),                // 52 - UART4
    Some(default_handler),                // 53 - UART5
    Some(default_handler),                // 54 - TIM6
    Some(default_handler),                // 55 - TIM7
    Some(default_handler),                // 56 - DMA2 channel 1
    Some(default_handler),                // 57 - DMA2 channel 2
    Some(default_handler),                // 58 - DMA2 channel 3
    Some(default_handler),                // 59 - DMA2 channel 4
    Some(default_handler),                // 60 - DMA2 channel 5
    Some(default_handler),                // 61 - ADC4
    None,                // 62 - RSVED
    None,                // 63 - RSVED
    Some(default_handler),                // 64 - COMP1 2 3
    Some(default_handler),                // 65 - COMP4 5 6
    Some(default_handler),                // 66 - COMP7
    None,                // 67 - RSVED
    None,                // 68 - RSVED
    None,                // 69 - RSVED
    None,                // 70 - RSVED
    None,                // 71 - RSVED
    Some(default_handler),                // 72 - I2C3 event
    Some(default_handler),                // 73 - I2C3 error
    Some(default_handler),                // 74 - USB_HP
    Some(default_handler),                // 75 - USB_LP
    Some(default_handler),                // 76 - USB wakeup
    Some(default_handler),                // 77 - TIM20 Break
    Some(default_handler),                // 78 - TIM20 Update
    Some(default_handler),                // 79 - TIM20 Trigger and Commutation
    Some(default_handler),                // 80 - TIM20 Capture Compare
    Some(default_handler),                // 81 - FPU
    None,                // 82 - RSVED
    None,                // 83 - RSVED
    Some(default_handler),                // 84 - SPI4
];

#[unsafe(no_mangle)]
fn nmi_handler() {
    // Handle NMI
    loop {
        //TODO: Implement NMI handling logic
    }
}

#[unsafe(no_mangle)]
fn hardfault_hander() {
    // Handle hard fault
    loop {
        //TODO: Implement hard fault handling logic
    }
}

#[unsafe(no_mangle)]
fn default_handler() {
    // Default handler for unhandled exceptions
}

//define reset handler
#[unsafe(no_mangle)]
fn reset_handler() {

    // Copy .data from flash to RAM


    // Initialize .bss to zero

    // Call main function
    crate::main();

}


//define exception and interrupt handlers


