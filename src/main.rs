#![no_main]
#![no_std]

mod drivers;
mod core;
mod system;

extern crate panic_halt;

use alloc::format;
use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

extern crate alloc;

use crate::system::system_clock_config;
use crate::drivers::gpio::*;
use stm32f1::stm32f103;
use crate::drivers::lcd_hd44780::Lcd;
use crate::drivers::serial::Serial;

use cortex_m::peripheral::NVIC;
use stm32f1::stm32f103::Interrupt::USART1;
use crate::core::delay::non_exact_time_delay;

#[entry]
fn main() -> !{

    system_clock_config();

    disable_jtag();

    //interrupts_init();

    let led1 = Gpio {port: GPIOPORT::GPIOC, pin_number: 13};
    led1.configure(GpioConfig {config: CONFIG::AnalogModeOrGeneralPurposeOutputPushPull, mode: MODE::Output10Mhz, pull: None});

    /*

    let lcd = Lcd {
        register_select: Gpio {port: GPIOPORT::GPIOA, pin_number: 15},
        read_write: None,
        enable: Gpio {port: GPIOPORT::GPIOB, pin_number: 3},
        d0: None,
        d1: None,
        d2: None,
        d3: None,
        d4: Gpio {port: GPIOPORT::GPIOB, pin_number: 6},
        d5: Gpio {port: GPIOPORT::GPIOB, pin_number: 7},
        d6: Gpio {port: GPIOPORT::GPIOB, pin_number: 8},
        d7: Gpio {port: GPIOPORT::GPIOB, pin_number: 9},
    };

    */

    let peripherals = stm32f103::Peripherals::take().unwrap();

    Serial::configure();
    Serial::println("STM32F103C8T6 rust serial test in USART1");

    let usart1_cr1;

    unsafe {
         usart1_cr1 = peripherals.USART1.cr1.as_ptr().read();
    }

    Serial::println(format!("USART1_CR1: {:X}", usart1_cr1).as_str());

    /*
    lcd.configure();
    lcd.set_cursor(1,1);
    lcd.print("rust embedded");

    */



    loop {

        let result = Serial::read_byte();

        if result != 0 {
            Serial::write_byte(result)
        }

        non_exact_time_delay(200);

    }
}

fn disable_jtag(){
    unsafe {
        let rcc = &*stm32f103::RCC::ptr();
        let afio = &*stm32f103::AFIO::ptr();

        rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 0));

        afio.mapr.as_ptr().write(afio.mapr.as_ptr().read() | (1 << 26));

    }
}

/*
fn interrupts_init(){
    unsafe {
        let mut nvic = cortex_m::Peripherals::take().unwrap().NVIC;

        NVIC::unmask(USART1);
        NVIC::set_priority(&mut nvic, USART1, 1);
    }
}
*/
