# RCC-DRIVE-LIB

![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![Platform](https://img.shields.io/badge/platform-Raspberry%20Pi-red?logo=raspberrypi)
![Crate](https://img.shields.io/badge/crate-rppal-blue)
![License](https://img.shields.io/badge/license-CC0-lightgrey)

---

## Description

`rcc-drive-lib` is a Rust library for controlling **ESC** and **RC servo motors** via hardware PWM on Raspberry Pi. It abstracts the low-level PWM duty-cycle math behind a clean, safe API, letting you drive brushless motors and steering servos with a single integer value.
Designed for embedded projects running on Raspberry Pi hardware.

---

## Features

- 🔌 **Hardware PWM control** via the `rppal` crate
- 🚗 **ESC throttle control**
- 🔧 **Servo steering control** with configurable trim offset (`pulse_offset`)
- 🛡️ **Safe neutral-on-drop** returns to the neutral pulse when dropped, preventing runaway actuators

---

## Prerequisites

- Raspberry Pi with hardware PWM support (this lib is tested on Pi3)
- Rust toolchain with the `aarch64-unknown-linux-gnu` for 64-bit or `armv7-unknown-linux-gnueabihf` for 32-bit target
- Hardware PWM enabled via `/boot/firmware/config.txt`:
  ```
  dtoverlay=pwm-2chan
  ```
  > see more in the kernel [https://github.com/raspberrypi/linux/blob/04c8e47067d4873c584395e5cb260b4f170a99ea/arch/arm/boot/dts/overlays/README#L944](docs)

---

## Installation

Add the crate to your `Cargo.toml`:

```bash
cargo add --path {path_to_lib}/rcc-drive-lib
```

Then build for your target platform. For cross-compilation from a Linux x86-64 host:

```bash
# Install the ARM target
rustup target add aarch64-unknown-linux-gnu

# Build
cargo build --release --target aarch64-unknown-linux-gnu
```

For native compilation directly on the Raspberry Pi:

```bash
cargo build --release
```

---

## Usage example

### ESC — Throttle Control

The `ESC` struct drives a brushless motor controller via **PWM channel 0** (`GPIO 12` / `GPIO 18` depending on overlay).

```rust
use rcc-drive-lib::ESC;

fn main() -> Result<(), rppal::pwm::Error>> {
    let esc = ESC::new(
        50.0,     // pwm_hz:       PWM frequency in Hz
        1000.0,   // pulse_min:    minimum pulse width in µs (full reverse)
        2000.0,   // pulse_max:    maximum pulse width in µs (full forward)
        1500.0,   // pulse_neutral: neutral/stop pulse width in µs
    )?;

    // set_throttle(value: i8) -> value is clamped from -127 up to 127
    // Full forward throttle
    esc.set_throttle(127)?;

    // 50% forward
    esc.set_throttle(64)?;

    // Neutral (stop)
    esc.set_throttle(0)?;

    // Full reverse
    esc.set_throttle(-127)?;

    // When `esc` is dropped, the PWM output automatically returns to neutral.
    Ok(())
}
```

### Servo — Steering Control

The `Servo` struct drives an RC servo via **PWM channel 1** (`GPIO 13` / `GPIO 19` depending on overlay).

```rust
use rcc-drive-lib::Servo;

fn main() -> Result<(), rppal::pwm::Error>> {
    let servo = Servo::new(
        50.0,     // pwm_hz:        PWM frequency in Hz
        1000.0,   // pulse_min:     minimum pulse width in µs (full left)
        2000.0,   // pulse_max:     maximum pulse width in µs (full right)
        1500.0,   // pulse_neutral: center pulse width in µs
        20.0,     // pulse_offset:  trim offset in µs (use 0.0 if no trim needed)
    )?;

    // set_steer(value: i8) -> value is clamped from -127 up to 127
    // Full right
    servo.set_steer(127)?;

    // Center
    servo.set_steer(0)?;

    // Full left
    servo.set_steer(-127)?;

    Ok(())
}
```

---

## License

This project is released under the **CC0 1.0 Universal**.  
You are free to copy, modify, distribute and use the work.

> See [LICENSE](LICENSE) for the full text.
