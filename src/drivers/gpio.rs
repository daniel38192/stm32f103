#![allow(dead_code)]

use core::cmp::PartialEq;
use stm32f1::stm32f103;

pub enum CONFIG {
    AnalogModeOrGeneralPurposeOutputPushPull = 0,
    FloatingInputOrGeneralPurposeOpenDrain = 1,
    InputWithPullUpPullDownOrAlternateFuncOutputPushPull = 2,
    AlternateFunctionOutputOpenDrain = 3,
}

impl PartialEq for CONFIG {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

pub enum MODE {
    Input = 0,
    Output10Mhz = 1,
    Output2Mhz = 2,
    Output50Mhz = 3
}

#[derive(Clone)]
pub enum PULL {
    PullDown = 0,
    PullUp = 1
}


#[derive(Clone)]
pub enum GPIOPORT {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
    GPIOG,
}

pub struct GpioConfig {
    pub config: CONFIG,
    pub mode: MODE,
    pub pull: Option<PULL>
}

impl GpioConfig {
    pub fn config_as_u32(&self) -> u32 {
        match self.config {
            CONFIG::AnalogModeOrGeneralPurposeOutputPushPull => 0,
            CONFIG::FloatingInputOrGeneralPurposeOpenDrain => 1,
            CONFIG::InputWithPullUpPullDownOrAlternateFuncOutputPushPull => 2,
            CONFIG::AlternateFunctionOutputOpenDrain => 3
        }
    }

    pub fn mode_as_u32(&self) -> u32 {
        match self.mode {
            MODE::Input => 0,
            MODE::Output10Mhz => 1,
            MODE::Output2Mhz => 2,
            MODE::Output50Mhz => 3
        }
    }

    pub fn pull_as_u32(&self) -> u32 {

        if self.pull.is_none() {
            return 0;
        }

        let pull = self.pull.clone().unwrap();

        match pull {
            PULL::PullDown => 0,
            PULL::PullUp => 1,
        }
    }
}

#[derive(Clone)]
pub struct Gpio {
    pub port: GPIOPORT,
    pub pin_number: u32
}

impl Gpio {
    pub fn high(&self){
        self.set(true)
    }

    pub fn low(&self){
        self.set(false)
    }

    pub fn set(&self, value: bool){
        match self.port {
            GPIOPORT::GPIOA => turn_in_gpio_a(self.pin_number, value),
            GPIOPORT::GPIOB => turn_in_gpio_b(self.pin_number, value),
            GPIOPORT::GPIOC => turn_in_gpio_c(self.pin_number, value),
            _ => {}
        }
    }

    pub fn configure(&self, config: GpioConfig){
        self.enable_in_rcc();
        match self.port {
            GPIOPORT::GPIOA => configure_in_gpio_a(config, self.pin_number),
            GPIOPORT::GPIOB => configure_in_gpio_b(config, self.pin_number),
            GPIOPORT::GPIOC => configure_in_gpio_c(config, self.pin_number),
            _ => {}
        }
    }

    fn enable_in_rcc(&self){
        unsafe  {
            let rcc = &*stm32f103::RCC::ptr();
            match self.port {
                GPIOPORT::GPIOA => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 2)),
                GPIOPORT::GPIOB => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 3)),
                GPIOPORT::GPIOC => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 4)),
                GPIOPORT::GPIOD => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 5)),
                GPIOPORT::GPIOE => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 6)),
                GPIOPORT::GPIOF => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 7)),
                GPIOPORT::GPIOG => rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 8)),
            }
        }
    }
}

fn turn_in_gpio_a(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32f103::GPIOA::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}

fn turn_in_gpio_b(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32f103::GPIOB::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}

fn turn_in_gpio_c(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32f103::GPIOC::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}

fn configure_in_gpio_a(config: GpioConfig, pin_number: u32){
    unsafe  {
        let port = &*stm32f103::GPIOA::ptr();

        if pin_number > 7 {
            // We begin by first clearing every gpio config.
            port.crh.as_ptr().write(port.crh.as_ptr().read() & !(0xF << ((pin_number * 4) - 32)));
            // Set config CNFy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.config_as_u32() << (((pin_number * 4) - 32) + 2)));
            // Set mode MODEy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.mode_as_u32() << ((pin_number * 4) - 32)))
        } else {
            // We begin by first clearing every gpio config.
            port.crl.as_ptr().write(port.crl.as_ptr().read() & !(0xF << (pin_number * 4)));
            // Set config CNFy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.config_as_u32() << ((pin_number * 4) + 2)));
            // Set mode MODEy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.mode_as_u32() << (pin_number * 4)))
        }

        // Set pull up / pull down respectively in Gpio ODR register.
        if (config.pull.is_some()) && (config.config == CONFIG::InputWithPullUpPullDownOrAlternateFuncOutputPushPull) {
            port.odr.as_ptr().write(port.odr.as_ptr().read() | (config.pull_as_u32() << pin_number))
        }
    }
}

fn configure_in_gpio_b(config: GpioConfig, pin_number: u32){
    unsafe  {
        let port = &*stm32f103::GPIOB::ptr();

        if pin_number > 7 {
            // We begin by first clearing every gpio config.
            port.crh.as_ptr().write(port.crh.as_ptr().read() & !(0xF << ((pin_number * 4) - 32)));
            // Set config CNFy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.config_as_u32() << (((pin_number * 4) - 32) + 2)));
            // Set mode MODEy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.mode_as_u32() << ((pin_number * 4) - 32)))
        } else {
            // We begin by first clearing every gpio config.
            port.crl.as_ptr().write(port.crl.as_ptr().read() & !(0xF << (pin_number * 4)));
            // Set config CNFy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.config_as_u32() << ((pin_number * 4) + 2)));
            // Set mode MODEy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.mode_as_u32() << (pin_number * 4)))
        }

        // Set pull up / pull down respectively in Gpio ODR register.
        if (config.pull.is_some()) && (config.config == CONFIG::InputWithPullUpPullDownOrAlternateFuncOutputPushPull) {
            port.odr.as_ptr().write(port.odr.as_ptr().read() | (config.pull_as_u32() << pin_number))
        }
    }
}

fn configure_in_gpio_c(config: GpioConfig, pin_number: u32){
    unsafe  {
        let port = &*stm32f103::GPIOC::ptr();

        if pin_number > 7 {
            // We begin by first clearing every gpio config.
            port.crh.as_ptr().write(port.crh.as_ptr().read() & !(0xF << ((pin_number * 4) - 32)));
            // Set config CNFy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.config_as_u32() << (((pin_number * 4) - 32) + 2)));
            // Set mode MODEy[1:0].
            port.crh.as_ptr().write(port.crh.as_ptr().read() | (config.mode_as_u32() << ((pin_number * 4) - 32)))
        } else {
            // We begin by first clearing every gpio config.
            port.crl.as_ptr().write(port.crl.as_ptr().read() & !(0xF << (pin_number * 4)));
            // Set config CNFy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.config_as_u32() << ((pin_number * 4) + 2)));
            // Set mode MODEy[1:0].
            port.crl.as_ptr().write(port.crl.as_ptr().read() | (config.mode_as_u32() << (pin_number * 4)))
        }

        // Set pull up / pull down respectively in Gpio ODR register.
        if (config.pull.is_some()) && (config.config == CONFIG::InputWithPullUpPullDownOrAlternateFuncOutputPushPull) {
            port.odr.as_ptr().write(port.odr.as_ptr().read() | (config.pull_as_u32() << pin_number))
        }
    }
}
