use stm32f1::stm32f103;

#[allow(dead_code)]
pub const APB1_FREQUENCY: u32 = 36000000;
#[allow(dead_code)]
pub const APB2_FREQUENCY: u32 = 72000000;
#[allow(dead_code)]
pub const SYSCLK_FREQUENCY: u32 = 72000000;

pub fn system_clock_config(){

    unsafe {
        let rcc = &*stm32f103::RCC::ptr();

        let flash = &*stm32f103::FLASH::ptr();

        // Enable HSI clock and set RCC_CR in default state
        rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 0));

        // Wait for HSI ready flag
        while!((rcc.cr.as_ptr().read() & (1 << 1)) > 0) { }


        // Configuration
        // External clock 8mhz Crystal/Ceramic resonator
        // Using max clock speed
        // System Running at 72Mhz
        // PCLK1 running at 36Mhz
        // PCLK2 running at 72mhz


        // 1. Enable HSE clock in RCC_CR bit 16
        rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 16));

        // 2. Wait for HSE RDY flag bit 17
        while!((rcc.cr.as_ptr().read() & (1 << 17)) > 0) { }

        // 3. Set HSE oscillator as PLL entry clock source in RCC_CFGR bit 16
        rcc.cfgr.as_ptr().write(rcc.cfgr.as_ptr().read() | (1 << 16));

        // 4. Set PLL multiplication factor in Bits 21:18 RCC_CFGR   7
        rcc.cfgr.as_ptr().write(rcc.cfgr.as_ptr().read() | (7 << 18));

        // 5. Enable PLL clock in RCC_CR Bit 24
        rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 24));

        // 6. Wait for PLL RDY flag Bit 25 RCC_CR
        while!((rcc.cr.as_ptr().read() & (1 << 25)) > 0) { }

        // 7. Set flash latency to 2 wait states in FLASH_ACR register
        flash.acr.as_ptr().write(flash.acr.as_ptr().read() | (2 << 0));

        // 8. Set APB low-speed prescaler to divided by 2 in Bits 10:8 (APB1) RCC_CFGR
        rcc.cfgr.as_ptr().write(rcc.cfgr.as_ptr().read() | (4 << 8));

        // 9. Select PLL as System clock in System clock switch in bits 1:0 RCC_CFGR
        rcc.cfgr.as_ptr().write(rcc.cfgr.as_ptr().read() | (2 << 0));

        // 10. Wait for PLL to be selected as System clock, System clock switch status Bits 3:2 in RCC_CFGR
        while!((rcc.cfgr.as_ptr().read() & (2 << 2)) > 0) { }

        // System is running at max clock speed (72Mhz)

    }
}