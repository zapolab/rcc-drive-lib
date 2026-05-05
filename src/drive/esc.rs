use rppal::pwm::{self, Channel, Polarity, Pwm};

use crate::drive::pulse_to_duty;

pub struct ESC {
    pwm_hz: f64,
    pulse_min: f64,
    pulse_max: f64,
    pulse_neutral: f64,
    pwm: Pwm,
}

impl ESC {
    pub fn new(
        pwm_hz: f64,
        pulse_min: f64,
        pulse_max: f64,
        pulse_neutral: f64,
    ) -> Result<Self, rppal::pwm::Error> {
        let esc = Self {
            pwm_hz,
            pulse_min,
            pulse_max,
            pulse_neutral,
            pwm: Pwm::with_frequency(
                Channel::Pwm0,
                pwm_hz,
                pulse_to_duty(pulse_neutral, pwm_hz),
                Polarity::Normal,
                true,
            )?,
        };

        Ok(esc)
    }

    pub fn set_throttle(&self, value: i8) -> Result<(), pwm::Error> {
        let value = f64::from(value.clamp(-127, 127)) / 127.0;

        let pulse = if value >= 0.0 {
            self.pulse_neutral + value * (self.pulse_max - self.pulse_neutral)
        } else {
            self.pulse_neutral - value * (self.pulse_min - self.pulse_neutral)
        };

        self.pwm.set_duty_cycle(pulse_to_duty(pulse, self.pwm_hz))
    }
}

impl Drop for ESC {
    fn drop(&mut self) {
        let result = self
            .pwm
            .set_duty_cycle(pulse_to_duty(self.pulse_neutral, self.pwm_hz));

        if let Err(e) = result {
            eprintln!("{e}");
        }
    }
}
