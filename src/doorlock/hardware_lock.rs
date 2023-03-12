use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::gpio::Result;
use std::thread;
use std::time::Duration;

use super::DoorLockTrait;

const GPIO_PWM: u8 = 23;
const GPIO_REED_SWITCH: u8 = 24;

pub struct DoorLock {
    servo_pin: u8,
    reed_switch_pin: u8,
}

impl DoorLockTrait for DoorLock {
    fn new() -> Self {
        Self {
            servo_pin: GPIO_PWM,
            reed_switch_pin: GPIO_REED_SWITCH,
        }
    }

    fn open(&mut self) {
        let servo_pin = Gpio::new()
            .unwrap()
            .get(self.servo_pin)
            .unwrap()
            .into_output();
        let mut servo = Servo::new(servo_pin);
        servo.rotate(0).unwrap();
        thread::sleep(Duration::from_millis(2000));
        servo.rotate(128).unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn close(&mut self) {
        let servo_pin = Gpio::new()
            .unwrap()
            .get(self.servo_pin)
            .unwrap()
            .into_output();
        let mut servo = Servo::new(servo_pin);
        servo.rotate(255).unwrap();
        thread::sleep(Duration::from_millis(2000));
        servo.rotate(128).unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn is_open(&self) -> bool {
        let switch = Gpio::new()
            .unwrap()
            .get(self.reed_switch_pin)
            .unwrap()
            .into_input_pulldown();
        switch.is_low()
    }
}

pub struct Servo {
    servo_pin: OutputPin,
}

impl Servo {
    const PERIOD_MS: u64 = 20;
    const PULSE_MIN_US: u64 = 600;
    const PULSE_MAX_US: u64 = 2300;

    pub fn new(servo_pin: OutputPin) -> Self {
        Self { servo_pin }
    }

    const fn angle_to_millis(angle: u8) -> u64 {
        ((Self::PULSE_MAX_US - Self::PULSE_MIN_US) * angle as u64) / 255 + Self::PULSE_MIN_US
    }

    pub fn rotate(&mut self, angle: u8) -> Result<()> {
        let dur = Self::angle_to_millis(angle);

        self.servo_pin.set_pwm(
            Duration::from_millis(Self::PERIOD_MS),
            Duration::from_micros(dur),
        )
    }
}
