//define vector table

static VECTOR_TABLE: [Option<unsafe fn()>; 146] = [
    //16 interrupt vectors
    None,                // 0x0 Initial stack pointer
    Some(reset_handler), // 0x4 Reset handler
    Some(nmi_handler),                // 0x8 NMI handler
    Some(hardfault_hander),                // 0xC none Secure or nonsecure Hard fault handler
    Some(mem_manage_handler),                // 0x10 Memory management fault handler
    Some(busfault_handler),                // 0x14 Bus fault handler
    Some(usagefault_handler),                // 0x18 Usage fault handler
    Some(secfault_handler),                // 0x1C Sec Fault handler
    None,                // 0x20 Reserved
    None,                // 0x24 Reserved
    None,                // 0x28 Reserved
    Some(svc_handler),   // 0x2C SVC via SWI
    Some(debugmon_handler),                // 0x30 Debug monitor handler
    None,                // 0x34 Reserved
    Some(default_handler),                // 0x38 PendSV handler
    Some(default_handler),                // 0x3C SysTick handler
    //130 external interrupts
    Some(WWDG_Handler),           // 0 - WWDG_Handler
    None,                        // 1 - None
    None,                        // 2 - None
    None,                        // 3 - None
    Some(TAMP_Handler),          // 4 - TAMP_Handler
    Some(RAMCFG_Handler),        // 5 - RAMCFG_Handler
    Some(FLASH_Handler),         // 6 - FLASH_Handler
    Some(FLASH_S_Handler),       // 7 - FLASH_S_Handler
    Some(GTZC_Handler),          // 8 - GTZC_Handler
    Some(RCC_Handler),           // 9 - RCC_Handler
    Some(RCC_S_Handler),         // 10 - RCC_S_Handler
    Some(EXTI0_Handler),         // 11 - EXTI0_Handler
    Some(EXTI1_Handler),         // 12 - EXTI1_Handler
    Some(EXTI2_Handler),         // 13 - EXTI2_Handler
    Some(EXTI3_Handler),         // 14 - EXTI3_Handler
    Some(EXTI4_Handler),         // 15 - EXTI4_Handler
    Some(EXTI5_Handler),         // 16 - EXTI5_Handler
    Some(EXTI6_Handler),         // 17 - EXTI6_Handler
    Some(EXTI7_Handler),         // 18 - EXTI7_Handler
    Some(EXTI8_Handler),         // 19 - EXTI8_Handler
    Some(EXTI9_Handler),         // 20 - EXTI9_Handler
    Some(EXTI10_Handler),        // 21 - EXTI10_Handler
    Some(EXTI11_Handler),        // 22 - EXTI11_Handler
    Some(EXTI12_Handler),        // 23 - EXTI12_Handler
    Some(EXTI13_Handler),        // 24 - EXTI13_Handler
    Some(EXTI14_Handler),        // 25 - EXTI14_Handler
    Some(EXTI15_Handler),        // 26 - EXTI15_Handler
    Some(GPDMA1_CH0_Handler),    // 27 - GPDMA1_CH0_Handler
    Some(GPDMA1_CH1_Handler),    // 28 - GPDMA1_CH1_Handler
    Some(GPDMA1_CH2_Handler),    // 29 - GPDMA1_CH2_Handler
    Some(GPDMA1_CH3_Handler),    // 30 - GPDMA1_CH3_Handler
    Some(GPDMA1_CH4_Handler),    // 31 - GPDMA1_CH4_Handler
    Some(GPDMA1_CH5_Handler),    // 32 - GPDMA1_CH5_Handler
    Some(GPDMA1_CH6_Handler),    // 33 - GPDMA1_CH6_Handler
    Some(GPDMA1_CH7_Handler),    // 34 - GPDMA1_CH7_Handler
    Some(IWDG_Handler),          // 35 - IWDG_Handler
    None,                        // 36 - None
    Some(ADC1_Handler),          // 37 - ADC1_Handler
    Some(DAC1_Handler),          // 38 - DAC1_Handler
    Some(FDCAN1_IT0_Handler),    // 39 - FDCAN1_IT0_Handler
    Some(FDCAN1_IT1_Handler),    // 40 - FDCAN1_IT1_Handler
    Some(TIM1_BRK_TERR_IERR_Handler), // 41 - TIM1_BRK_TERR_IERR_Handler
    Some(TIM1_UP_Handler),       // 42 - TIM1_UP_Handler
    Some(TIM1_TRG_COM_DIR_IDX_Handler), // 43 - TIM1_TRG_COM_DIR_IDX_Handler
    Some(TIM1_CC_Handler),       // 44 - TIM1_CC_Handler
    Some(TIM2_Handler),          // 45 - TIM2_Handler
    Some(TIM3_Handler),          // 46 - TIM3_Handler
    Some(TIM4_Handler),          // 47 - TIM4_Handler
    Some(TIM5_Handler),          // 48 - TIM5_Handler
    Some(TIM6_Handler),          // 49 - TIM6_Handler
    Some(TIM7_Handler),          // 50 - TIM7_Handler
    Some(I2C1_EV_Handler),       // 51 - I2C1_EV_Handler
    Some(I2C1_ER_Handler),       // 52 - I2C1_ER_Handler
    Some(I2C2_EV_Handler),       // 53 - I2C2_EV_Handler
    Some(I2C2_ER_Handler),       // 54 - I2C2_ER_Handler
    Some(SPI1_Handler),          // 55 - SPI1_Handler
    Some(SPI2_Handler),          // 56 - SPI2_Handler
    Some(SPI3_Handler),          // 57 - SPI3_Handler
    Some(USART1_Handler),        // 58 - USART1_Handler
    Some(USART2_Handler),        // 59 - USART2_Handler
    Some(USART3_Handler),        // 60 - USART3_Handler
    Some(UART4_Handler),         // 61 - UART4_Handler
    Some(UART5_Handler),         // 62 - UART5_Handler
    Some(LPUART1_Handler),       // 63 - LPUART1_Handler
    Some(LPTIM1_Handler),        // 64 - LPTIM1_Handler
    Some(IM8_BRK_TERR_IERR_Handler), // 65 - IM8_BRK_TERR_IERR_Handler
    Some(TIM8_UP_Handler),       // 66 - TIM8_UP_Handler
    Some(TIM8_TRG_COM_DIR_IDX_Handler), // 67 - TIM8_TRG_COM_DIR_IDX_Handler
    Some(TIM8_CC_Handler),       // 68 - TIM8_CC_Handler
    Some(ADC2_Handler),          // 69 - ADC2_Handler
    Some(LPTIM2_Handler),        // 70 - LPTIM2_Handler
    Some(TIM15_Handler),         // 71 - TIM15_Handler
    Some(TIM16_Handler),         // 72 - TIM16_Handler
    Some(TIM17_Handler),         // 73 - TIM17_Handler
    Some(USB_FS_Handler),        // 74 - USB_FS_Handler
    Some(CRS_Handler),           // 75 - CRS_Handler
    Some(UCPD1_Handler),         // 76 - UCPD1_Handler
    Some(FMC_Handler),           // 77 - FMC_Handler
    Some(OCTOSPI1_Handler),      // 78 - OCTOSPI1_Handler
    Some(SDMMC1_Handler),        // 79 - SDMMC1_Handler
    Some(I2C3_EV_Handler),       // 80 - I2C3_EV_Handler
    Some(I2C3_ER_Handler),       // 81 - I2C3_ER_Handler
    Some(SPI4_Handler),          // 82 - SPI4_Handler
    Some(SPI5_Handler),          // 83 - SPI5_Handler
    Some(SPI6_Handler),          // 84 - SPI6_Handler
    Some(USART6_Handler),        // 85 - USART6_Handler
    Some(USART10_Handler),       // 86 - USART10_Handler
    Some(USART11_Handler),       // 87 - USART11_Handler
    Some(SAI1_Handler),          // 88 - SAI1_Handler
    Some(SAI2_Handler),          // 89 - SAI2_Handler
    Some(GPDMA2_CH0_Handler),    // 90 - GPDMA2_CH0_Handler
    Some(GPDMA2_CH1_Handler),    // 91 - GPDMA2_CH1_Handler
    Some(GPDMA2_CH2_Handler),    // 92 - GPDMA2_CH2_Handler
    Some(GPDMA2_CH3_Handler),    // 93 - GPDMA2_CH3_Handler
    Some(GPDMA2_CH4_Handler),    // 94 - GPDMA2_CH4_Handler
    Some(GPDMA2_CH5_Handler),    // 95 - GPDMA2_CH5_Handler
    Some(GPDMA2_CH6_Handler),    // 96 - GPDMA2_CH6_Handler
    Some(GPDMA2_CH7_Handler),    // 97 - GPDMA2_CH7_Handler
    Some(UART7_Handler),         // 98 - UART7_Handler
    Some(UART8_Handler),         // 99 - UART8_Handler
    Some(UART9_Handler),         // 100 - UART9_Handler
    Some(UART12_Handler),        // 101 - UART12_Handler
    Some(SDMMC2_Handler),        // 102 - SDMMC2_Handler
    None,                        // 103 - None
    Some(ICACHE_Handler),        // 104 - ICACHE_Handler
    Some(DCACHE_Handler),        // 105 - DCACHE_Handler
    Some(ETH_Handler),           // 106 - ETH_Handler
    Some(ETH_WKUP_Handler),      // 107 - ETH_WKUP_Handler
    Some(DCMI_PSSI_Handler),     // 108 - DCMI_PSSI_Handler
    Some(FDCAN2_IT0_Handler),    // 109 - FDCAN2_IT0_Handler
    Some(FDCAN2_IT1_Handler),    // 110 - FDCAN2_IT1_Handler
    Some(Cordic_Handler),        // 111 - Cordic_Handler
    Some(FMAC_Handler),          // 112 - FMAC_Handler
    Some(DTS_WKUP_Handler),      // 113 - DTS_WKUP_Handler
    Some(RNG_Handler),           // 114 - RNG_Handler
    None,                        // 115 - None
    None,                        // 116 - None
    Some(HASH_Handler),          // 117 - HASH_Handler
    None,                        // 118 - None
    None,                        // 119 - None
    Some(TIM12_Handler),         // 120 - TIM12_Handler
    Some(TIM13_Handler),         // 121 - TIM13_Handler
    Some(TIM14_Handler),         // 122 - TIM14_Handler
    Some(I3C1_EV_Handler),       // 123 - I3C1_EV_Handler
    Some(I3C1_ER_Handler),       // 124 - I3C1_ER_Handler
    Some(I2C4_EV_Handler),       // 125 - I2C4_EV_Handler
    Some(I2C4_ER_Handler),       // 126 - I2C4_ER_Handler
    Some(LPTIM3_Handler),        // 127 - LPTIM3_Handler
    Some(LPTIM4_Handler),        // 128 - LPTIM4_Handler
    Some(LPTIM5_Handler),        // 129 - LPTIM5_Handler
    Some(LPTIM6_Handler),        // 130
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


