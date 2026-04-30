use rppal::pwm::{self, Channel, Polarity, Pwm};

fn pulse_to_duty(pulse: f64, pwm_hz: f64) -> f64 {
    pulse / (1_000_000.0 / pwm_hz)
}

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

    pub fn set_throttle(&self, value: i16) -> Result<(), pwm::Error> {
        let value = f64::from(value.clamp(-255, 255)) / 255.0;

        let pulse = if value >= 0.0 {
            self.pulse_neutral + value * (self.pulse_max - self.pulse_neutral)
        } else {
            self.pulse_neutral - (value.abs()) * (self.pulse_neutral - self.pulse_min)
        };

        self.pwm.set_duty_cycle(pulse_to_duty(pulse, self.pwm_hz))
    }
}

impl Drop for ESC {
    fn drop(&mut self) {
        let res = self
            .pwm
            .set_duty_cycle(pulse_to_duty(self.pulse_neutral, self.pwm_hz));

        match res {
            Ok(()) => (),
            Err(e) => println!("{e}"),
        }
    }
}

pub struct Servo {
    pwm_hz: f64,
    pulse_min: f64,
    pulse_max: f64,
    pulse_neutral: f64,
    pulse_offset: f64,
    pwm: Pwm,
}

impl Servo {
    pub fn new(
        pwm_hz: f64,
        pulse_min: f64,
        pulse_max: f64,
        pulse_neutral: f64,
        pulse_offset: f64,
    ) -> Result<Self, rppal::pwm::Error> {
        let servo = Self {
            pwm_hz,
            pulse_min,
            pulse_max,
            pulse_neutral,
            pulse_offset,
            pwm: Pwm::with_frequency(
                Channel::Pwm1,
                pwm_hz,
                pulse_to_duty(pulse_neutral, pwm_hz),
                Polarity::Normal,
                true,
            )?,
        };

        Ok(servo)
    }

    pub fn set_steer(&self, value: i16) -> Result<(), pwm::Error> {
        let value = f64::from(value.clamp(-255, 255)) / 255.0;

        let pulse = if value >= 0.0 {
            (self.pulse_neutral + self.pulse_offset) + value * (self.pulse_max - self.pulse_neutral)
        } else {
            (self.pulse_neutral + self.pulse_offset)
                - (value.abs()) * (self.pulse_neutral - self.pulse_min)
        };

        self.pwm.set_duty_cycle(pulse_to_duty(pulse, self.pwm_hz))
    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        let res = self.pwm.set_duty_cycle(pulse_to_duty(
            self.pulse_neutral + self.pulse_offset,
            self.pwm_hz,
        ));

        if let Err(e) = res {
            println!("{e}");
        }
    }
}
