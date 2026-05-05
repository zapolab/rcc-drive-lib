pub mod esc;
pub mod servo;

#[must_use]
pub fn pulse_to_duty(pulse: f64, pwm_hz: f64) -> f64 {
    pulse * pwm_hz / 1_000_000.0
}
