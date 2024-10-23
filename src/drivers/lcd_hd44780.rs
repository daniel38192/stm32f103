#![allow(dead_code)]

use crate::core::delay::non_exact_time_delay;
use crate::drivers::gpio::*;

pub struct Lcd{
    pub register_select: Gpio,
    pub read_write: Option<Gpio>, //Optional Pin
    pub enable: Gpio,
    pub d0: Option<Gpio>, //Optional Pin (in 4 bit mode)
    pub d1: Option<Gpio>, //Optional Pin (in 4 bit mode)
    pub d2: Option<Gpio>, //Optional Pin (in 4 bit mode)
    pub d3: Option<Gpio>, //Optional Pin (in 4 bit mode)
    pub d4: Gpio,
    pub d5: Gpio,
    pub d6: Gpio,
    pub d7: Gpio,
}

const TYPE_GPIO_LCD_PORT: GpioConfig = GpioConfig {
    config: CONFIG::AnalogModeOrGeneralPurposeOutputPushPull,
    mode: MODE::Output50Mhz,
    pull: None,
};

impl Lcd {
    pub fn configure(&self){

        self.init_pins();
        //400000
        non_exact_time_delay(2400000);
        self.send_init_commands();

    }

    pub fn print(&self, str: &str){
        let bytes = str.as_bytes();

        for c in bytes {
            let b = *c as char;
            self.send_character(b);
        }
    }

    pub fn set_cursor(&self, x: i32, y: i32){
        self.send_command(0b10000000u8 + (x as u8 -1) + ((y as u8-1)* 64));
    }

    pub fn send_character(&self, c: char){
        if (self.d0.is_none()) | (self.d1.is_none()) | (self.d2.is_none()) | (self.d3.is_none()) {
            self.send_character_8_bit((c as u8 >> 4) as char);
            self.send_character_8_bit((c as u8 & 0xF ) as char);
        } else {
            self.send_character_8_bit(c);
        }
    }

    pub fn send_command(&self, cmd: u8){
        if (self.d0.is_none()) | (self.d1.is_none()) | (self.d2.is_none()) | (self.d3.is_none()) {
            self.send_command_8_bit(cmd >> 4);
            self.send_command_8_bit(cmd & 0xF);
        } else {
            self.send_command_8_bit(cmd);
        }
    }

    fn send_init_commands(&self) {
        if (self.d0.is_none()) | (self.d1.is_none()) | (self.d2.is_none()) | (self.d3.is_none()) {
            self.send_command_8_bit(0x2);

            self.send_command(0x28);

            self.send_command(0xE);

            self.send_command(0x6);

            //self.send_command(0x80);

        } else {
            self.send_command(0x38);
            self.send_command(0xE);
            self.send_command(0x6);
        }

    }

    fn send_character_8_bit(&self, c: char){
        // RS -> 1
        // RW -> 0
        if self.read_write.is_some() {
            let rw = self.read_write.clone().unwrap();
            rw.low()
        };
        self.register_select.high();

        // first pause
        //400
        non_exact_time_delay(2400);

        // EN -> 1
        // c -> D0 - D7
        self.enable.high();
        self.toggle_pins_char(c);

        // second pause
        //800
        non_exact_time_delay(3200);

        // EN -> 0
        self.enable.low();

        // third pause
        //200
        non_exact_time_delay(1200);

    }

    fn send_command_8_bit(&self, cmd: u8){
        // RS -> 0
        // RW -> 0
        if self.read_write.is_some() {
            let rw = self.read_write.clone().unwrap();
            rw.low()
        };
        self.register_select.low();

        // first pause
        //400
        non_exact_time_delay(2400);

        // EN -> 1
        // c -> D0 - D7
        self.enable.high();
        self.toggle_pins(cmd);

        // second pause
        //800
        non_exact_time_delay(3200);

        // EN -> 0
        self.enable.low();

        // third pause
        //200
        non_exact_time_delay(1200);

    }

    pub fn clear(&self){
        self.send_command(0x1);
    }

    fn toggle_pins_char(&self, data: char){
        self.toggle_pins(data as u8)
    }

    pub fn toggle_pins(&self, data: u8) {
        if (self.d0.is_none()) | (self.d1.is_none()) | (self.d2.is_none()) | (self.d3.is_none()) {
            self.toggle_pins_4_bit(data)
        } else {
            self.toggle_pins_8_bit(data);
        }
    }

    fn toggle_pins_8_bit(&self, data: u8) {
        if (data & 0b00000001) > 0 {
            self.d0.clone().unwrap().high()
        } else {
            self.d0.clone().unwrap().low()
        }
        if (data & 0b00000010) > 0 {
            self.d1.clone().unwrap().high()
        } else {
            self.d1.clone().unwrap().low()
        }
        if (data & 0b00000100) > 0 {
            self.d2.clone().unwrap().high()
        } else {
            self.d2.clone().unwrap().low()
        }
        if (data & 0b00001000) > 0 {
            self.d3.clone().unwrap().high()
        } else {
            self.d3.clone().unwrap().low()
        }
        if (data & 0b00010000) > 0 {
            self.d4.high()
        } else {
            self.d4.low()
        }
        if (data & 0b00100000) > 0 {
            self.d5.high()
        } else {
            self.d5.low()
        }
        if (data & 0b01000000) > 0 {
            self.d6.high()
        } else {
            self.d6.low()
        }
        if (data & 0b10000000) > 0 {
            self.d7.high()
        } else {
            self.d7.low()
        }
    }

    fn toggle_pins_4_bit(&self, data: u8) {

        if (data & 0b0001) > 0 {
            self.d4.high()
        } else {
            self.d4.low()
        }
        if (data & 0b0010) > 0 {
            self.d5.high()
        } else {
            self.d5.low()
        }
        if (data & 0b0100) > 0 {
            self.d6.high()
        } else {
            self.d6.low()
        }
        if (data & 0b1000) > 0 {
            self.d7.high()
        } else {
            self.d7.low()
        }
    }

   pub fn init_pins(&self){

       if (self.d0.is_none()) | (self.d1.is_none()) | (self.d2.is_none()) | (self.d3.is_none()) {
           self.init_4bit_pins()
       } else {
           self.d0.clone().unwrap().configure(TYPE_GPIO_LCD_PORT);
           self.d1.clone().unwrap().configure(TYPE_GPIO_LCD_PORT);
           self.d2.clone().unwrap().configure(TYPE_GPIO_LCD_PORT);
           self.d3.clone().unwrap().configure(TYPE_GPIO_LCD_PORT);
           self.init_4bit_pins()
       }
    }

   fn init_4bit_pins(&self){
       self.register_select.configure(TYPE_GPIO_LCD_PORT);
       if self.read_write.is_some() {
           self.read_write.clone().unwrap().configure(TYPE_GPIO_LCD_PORT);
       }
       self.enable.configure(TYPE_GPIO_LCD_PORT);
       self.d4.configure(TYPE_GPIO_LCD_PORT);
       self.d5.configure(TYPE_GPIO_LCD_PORT);
       self.d6.configure(TYPE_GPIO_LCD_PORT);
       self.d7.configure(TYPE_GPIO_LCD_PORT);
    }
}
