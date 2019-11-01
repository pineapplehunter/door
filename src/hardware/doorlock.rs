use crate::hardware::servo::Servo;
use rppal::gpio::{Error, Gpio};
use std::thread;
use std::time::Duration;

pub struct DoorLock {
    servo_pin: u8,
    reed_switch_pin: u8,
}

impl DoorLock {
    pub fn new(servo_pin: u8, reed_switch_pin: u8) -> Self {
        Self {
            servo_pin,
            reed_switch_pin,
        }
    }

    pub fn open(&mut self) -> Result<(), Error> {
        let servo_pin = Gpio::new()?.get(self.servo_pin)?.into_output();
        let mut servo = Servo::new(servo_pin);
        servo.rotate(0)?;
        thread::sleep(Duration::from_millis(2000));
        servo.rotate(128)?;
        thread::sleep(Duration::from_millis(2000));

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        let servo_pin = Gpio::new()?.get(self.servo_pin)?.into_output();
        let mut servo = Servo::new(servo_pin);
        servo.rotate(255)?;
        thread::sleep(Duration::from_millis(2000));
        servo.rotate(128)?;
        thread::sleep(Duration::from_millis(2000));

        Ok(())
    }

    pub fn is_open(&self) -> Result<bool, Error> {
        let switch = Gpio::new()?.get(self.reed_switch_pin)?.into_input_pulldown();
        Ok(switch.is_low())
    }
}
