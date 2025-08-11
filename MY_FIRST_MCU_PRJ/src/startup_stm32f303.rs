



//define vector table


unsafe extern "C" {
    fn HardFault_Hander();
    fn MemManage_Handler();
    fn BusFault_Handler();
    fn UsageFault_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn ADC1_2_Handler();
    fn ADC3_Handler();
    fn ADC4_Handler();
    fn CAN_RX1_Handler();
    fn CAN_SCE_Handler();
    fn COMP123_Handler();
    fn COMP456_Handler();
    fn COMP7_Handler();
    fn DMA1_CH1_Handler();
    fn DMA1_CH2_Handler();
    fn DMA1_CH3_Handler();
    fn DMA1_CH4_Handler();
    fn DMA1_CH5_Handler();
    fn DMA1_CH6_Handler();
    fn DMA1_CH7_Handler();
    fn DMA2_CH1_Handler();
    fn DMA2_CH2_Handler();
    fn DMA2_CH3_Handler();
    fn DMA2_CH4_Handler();
    fn DMA2_CH5_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_TSC_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FLASH_Handler();
    fn FMC_Handler();
    fn FPU_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_EXTI23_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_EXTI24_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn PVD_Handler();
    fn RCC_Handler();
    fn RTCAlarm_Handler();
    fn RTC_WKUP_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn SPI4_Handler();
    fn TAMP_STAMP_Handler();
    fn TIM1_BRK_TIM15_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_TIM17_Handler();
    fn TIM1_UP_TIM16_Handler();
    fn TIM20_BRK_Handler();
    fn TIM20_CC_Handler();
    fn TIM20_TRG_COM_Handler();
    fn TIM20_UP_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM6_DACUNDER_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_Handler();
    fn TIM8_UP_Handler();
    fn UART4_EXTI34_Handler();
    fn UART5_EXTI35_Handler();
    fn USART1_EXTI25_Handler();
    fn USART2_EXTI26_Handler();
    fn USART3_EXTI28_Handler();
    fn USB_HP_CAN_TX_Handler();
    fn USB_HP_Handler();
    fn USB_LP_CAN_RX0_Handler();
    fn USB_LP_Handler();
    fn USB_WKUP_EXTI_Handler();
    fn USB_WKUP_Handler();
    fn WWDG_Handler();
}

unsafe extern "C" {
    static _sidata: u32; /* start of .data Flash*/
    static _sdata: u32; /* start of .data in RAM */
    static _edata: u32; /* end of .data in RAM */
    static _sbss: u32;  /* start of .bss in RAM */
    static _ebss: u32;  /* end of .bss in RAM */
    }

#[unsafe(link_section = ".isr_vector")]
#[used]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 98] = [
    /* stack pointer - inserted by linker */
    Some(Reset_Handler),              // Reset handler
    Some(NMI_Handler),                // NMI handler
    Some(HardFault_Hander),           // Hard fault handler
    Some(MemManage_Handler),          // Memory management fault handler
    Some(BusFault_Handler),           // Bus fault handler
    Some(UsageFault_Handler),         // Usage fault handler
    None,                             // Reserved
    None,                             // Reserved
    None,                             // Reserved
    None,                             // Reserved
    Some(SVCall_Handler),             // SVCall handler
    Some(PendSV_Handler),             // PendSV handler
    Some(SysTick_Handler),            // SysTick handler
    Some(WWDG_Handler),               // 0 - Window watchdog
    Some(PVD_Handler),                // 1 - PVD through EXTI line detection
    Some(TAMP_STAMP_Handler),         // 2 - Tamper and time stamp
    Some(RTC_WKUP_Handler),           // 3 - RTC wakeup through EXTI line
    Some(FLASH_Handler),              // 4 - Flash
    Some(RCC_Handler),                // 5 - RCC
    Some(EXTI0_Handler),              // 6 - EXTI line 0
    Some(EXTI1_Handler),              // 7 - EXTI line 1
    Some(EXTI2_TSC_Handler),          // 8 - EXTI line 2
    Some(EXTI3_Handler),              // 9 - EXTI line 3
    Some(EXTI4_Handler),              // 10 - EXTI line 4
    Some(DMA1_CH1_Handler),           // 11 - DMA1 channel 1
    Some(DMA1_CH2_Handler),           // 12 - DMA1 channel 2
    Some(DMA1_CH3_Handler),           // 13 - DMA1 channel 3
    Some(DMA1_CH4_Handler),           // 14 - DMA1 channel 4
    Some(DMA1_CH5_Handler),           // 15 - DMA1 channel 5
    Some(DMA1_CH6_Handler),           // 16 - DMA1 channel 6
    Some(DMA1_CH7_Handler),           // 17 - DMA1 channel 7
    Some(ADC1_2_Handler),             // 18 - ADC1, ADC2
    Some(USB_HP_CAN_TX_Handler),      // 19 - USB high priority
    Some(USB_LP_CAN_RX0_Handler),     // 20 - USB low priority
    Some(CAN_RX1_Handler),            // 21 - CAN1 RX1
    Some(CAN_SCE_Handler),            // 22 - CAN1 SCE
    Some(EXTI9_5_Handler),            // 23 - EXTI line 9 to 5
    Some(TIM1_BRK_TIM15_Handler),     // 24 - TIM1 break
    Some(TIM1_UP_TIM16_Handler),      // 25 - TIM1 update
    Some(TIM1_TRG_COM_TIM17_Handler), // 26 - TIM1 trigger and commutation
    Some(TIM1_CC_Handler),            // 27 - TIM1 capture compare
    Some(TIM2_Handler),               // 28 - TIM2
    Some(TIM3_Handler),               // 29 - TIM3
    Some(TIM4_Handler),               // 30 - TIM4
    Some(I2C1_EV_EXTI23_Handler),     // 31 - I2C1 event
    Some(I2C1_ER_Handler),            // 32 - I2C1 error
    Some(I2C2_EV_EXTI24_Handler),     // 33 - I2C2 event
    Some(I2C2_ER_Handler),            // 34 - I2C2 error
    Some(SPI1_Handler),               // 35 - SPI1
    Some(SPI2_Handler),               // 36 - SPI2
    Some(USART1_EXTI25_Handler),      // 37 - USART1
    Some(USART2_EXTI26_Handler),      // 38 - USART2
    Some(USART3_EXTI28_Handler),      // 39 - USART3
    Some(EXTI15_10_Handler),          // 40 - EXTI line 15 to 10
    Some(RTCAlarm_Handler),           // 41 - RTC alarm through
    Some(USB_WKUP_Handler),           // 42 - USB wakeup from suspend
    Some(TIM8_BRK_Handler),           // 43 - TIM8 break
    Some(TIM8_UP_Handler),            // 44 - TIM8 update
    Some(TIM8_TRG_COM_Handler),       // 45 - TIM8 trigger and commutation
    Some(TIM8_CC_Handler),            // 46 - TIM8 capture compare
    Some(ADC3_Handler),               // 47 - ADC3
    Some(FMC_Handler),                // 48 - FMC
    None,                             // 49 - RSVED
    None,                             // 50 - RSVED
    Some(SPI3_Handler),               // 51 - SPI3
    Some(UART4_EXTI34_Handler),       // 52 - UART4
    Some(UART5_EXTI35_Handler),       // 53 - UART5
    Some(TIM6_DACUNDER_Handler),      // 54 - TIM6
    Some(TIM7_Handler),               // 55 - TIM7
    Some(DMA2_CH1_Handler),           // 56 - DMA2 channel 1
    Some(DMA2_CH2_Handler),           // 57 - DMA2 channel 2
    Some(DMA2_CH3_Handler),           // 58 - DMA2 channel 3
    Some(DMA2_CH4_Handler),           // 59 - DMA2 channel 4
    Some(DMA2_CH5_Handler),           // 60 - DMA2 channel 5
    Some(ADC4_Handler),               // 61 - ADC4
    None,                             // 62 - RSVED
    None,                             // 63 - RSVED
    Some(COMP123_Handler),            // 64 - COMP1 2 3
    Some(COMP456_Handler),            // 65 - COMP4 5 6
    Some(COMP7_Handler),              // 66 - COMP7
    None,                             // 67 - RSVED
    None,                             // 68 - RSVED
    None,                             // 69 - RSVED
    None,                             // 70 - RSVED
    None,                             // 71 - RSVED
    Some(I2C3_EV_Handler),            // 72 - I2C3 event
    Some(I2C3_ER_Handler),            // 73 - I2C3 error
    Some(USB_HP_Handler),             // 74 - USB_HP
    Some(USB_LP_Handler),             // 75 - USB_LP
    Some(USB_WKUP_EXTI_Handler),      // 76 - USB wakeup
    Some(TIM20_BRK_Handler),          // 77 - TIM20 Break
    Some(TIM20_UP_Handler),           // 78 - TIM20 Update
    Some(TIM20_TRG_COM_Handler),      // 79 - TIM20 Trigger and Commutation
    Some(TIM20_CC_Handler),           // 80 - TIM20 Capture Compare
    Some(FPU_Handler),                // 81 - FPU
    None,                             // 82 - RSVED
    None,                             // 83 - RSVED
    Some(SPI4_Handler),               // 84 - SPI4
];

#[unsafe(no_mangle)]
extern "C" fn NMI_Handler() {
    // Handle NMI
    loop {
        //TODO: Implement NMI handling logic
    }
}

#[unsafe(no_mangle)]
extern "C" fn HardFault_Handler() {
    // Handle hard fault
    loop {
        //TODO: Implement hard fault handling logic
    }
}

#[unsafe(no_mangle)]
extern "C" fn Default_Handler() {
    // Default handler for unhandled exceptions
}

//define reset handler
#[unsafe(no_mangle)]
extern "C" fn Reset_Handler() {

    // Copy .data from flash to RAM
    let src_is_flash: *const u32 = unsafe { &_sidata };
    let dest_is_ram: *const u32 = unsafe { &_sdata };

    // Initialize .bss to zero

    // Call main function
    crate::main();

}


//define exception and interrupt handlers


