#![allow(dead_code)]

use stm32f1::stm32f103;
use crate::drivers::gpio::*;
use crate::system::APB2_FREQUENCY;
use stm32f1::stm32f103::interrupt;
use libm::roundf;

pub struct Serial {
}

const SERIAL_TYPE_GPIO: GpioConfig = GpioConfig {
    config: CONFIG::InputWithPullUpPullDownOrAlternateFuncOutputPushPull,
    mode: MODE::Output50Mhz,
    pull: None,
};

impl Serial {

    pub fn configure(){
        Self::disable();
        Self::enable_clock();
        Self::configure_ports();
        Self::configure_registers();
        Self::set_baudrate_rs(115200);
        Self::enable();
        Serial::write_byte('\n' as u8);
    }

    pub fn println(s: &str){
        Self::print(s);
        Self::new_line()
    }

    pub fn print(s: &str){
        for &b in s.as_bytes() {
            Self::write_byte(b);
        }
    }

    pub fn new_line(){
        Self::write_byte(0xA); //New line
        Self::write_byte(0xD) //Carriage return
    }

    pub fn write_byte(byte: u8){
        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();
            // write byte in USART_DR register
            serial_port.dr.as_ptr().write(byte as u32);
            // wait for transmission complete bit 6 in USART_SR register
            while !((serial_port.sr.as_ptr().read() & (1 << 6)) > 0) { }
        }
    }

    pub fn read_byte() -> u8 {
        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();

            if (serial_port.sr.as_ptr().read() & (1 << 5)) > 0 {
                serial_port.dr.as_ptr().read() as u8
            } else {
                0
            }
        }
    }

    pub fn set_baudrate_rs(baudrate: u32){
        /*                   Fck
         * USARTDIV = ------------------
         *              16 * Tx / Rx baud
         */

        let usartdiv = APB2_FREQUENCY as f32 / (16.0 * baudrate as f32);
        let matissa = usartdiv as u32;
        let fraction = roundf((usartdiv - matissa as f32) * 16.0) as u32;

        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();
            // program matissa and fraction in USART_BRR register
            serial_port.brr.as_ptr().write((matissa << 4) | (fraction << 0));
        }
    }

    fn enable_clock(){
        unsafe {
            let rcc = &*stm32f103::RCC::ptr();
            // enable USART1 clock in rcc
            rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 14));
            // enable AFIO clock in rcc
            rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 0));
        }
    }

    fn configure_ports(){
        // USART1_TX PA9
        // USART1_RX PA10
        let tx_pin = Gpio {port: GPIOPORT::GPIOA, pin_number: 9};
        let rx_pin = Gpio {port: GPIOPORT::GPIOA, pin_number: 10};
        tx_pin.configure(SERIAL_TYPE_GPIO);
        rx_pin.configure(SERIAL_TYPE_GPIO);
        // no remap needed in AFIO_MAPR register for USART1 in this case
    }

    fn configure_registers(){
        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();
            // set default prescaler value
            serial_port.gtpr.as_ptr().write(serial_port.gtpr.as_ptr().read() | 1);
            // enable transmitter
            serial_port.cr1.as_ptr().write(serial_port.cr1.as_ptr().read() | (1 << 3));
            // enable receiver
            serial_port.cr1.as_ptr().write(serial_port.cr1.as_ptr().read() | (1 << 2));
            /*
            // enable RXNE interrupt bit 5
            serial_port.cr1.as_ptr().write(serial_port.cr1.as_ptr().read() | (1 << 5))
             */
        }
    }

    pub fn enable(){
        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();
            // enable usart
            serial_port.cr1.as_ptr().write(serial_port.cr1.as_ptr().read() | (1 << 13));
        }
    }

    pub fn disable(){
        unsafe {
            let serial_port = &*stm32f103::USART1::ptr();
            // disable usart
            serial_port.cr1.as_ptr().write(serial_port.cr1.as_ptr().read() & !(1 << 13));
        }
    }
}

/*
#[interrupt]
fn USART1() {
    Serial::println(format!("Received: {}", Serial::read_byte()).as_str())
}
*/