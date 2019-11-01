use rppal::gpio::OutputPin;
use rppal::gpio::Result;
use std::time::Duration;

pub struct Servo {
    servo_pin: OutputPin,
}

impl Servo {
    const PERIOD_MS: u64 = 20;
    const PULSE_MIN_US: u64 = 600;
    const PULSE_MAX_US: u64 = 2300;

    pub fn new(servo_pin: OutputPin) -> Self {
        Self {
            servo_pin,
        }
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
