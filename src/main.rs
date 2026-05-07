#![no_std]
#![no_main]

#![allow(unused_imports)]

use panic_halt;

use stm32f4xx_hal as hal;

use hal::pac;

use hal::prelude::_stm32f4xx_hal_rcc_RccExt;
use hal::prelude::_stm32f4xx_hal_gpio_GpioExt;
use hal::prelude::_fugit_RateExtU32;

use hal::rcc::Config;

use hal::hal_02::digital::v2::OutputPin;
use cortex_m::asm::delay;

pub fn fake_exit() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

pub fn fake_debug_exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _dp = pac::Peripherals::take().expect("cannot take peripherals");
    let _cp = pac::CorePeripherals::take().expect("cannot take core peripherals");

    let config = Config::hse(25.MHz()).sysclk(84.MHz()).require_pll48clk();
    let mut rcc = _dp.RCC.freeze(config);

    let _gpio_a = _dp.GPIOA.split(&mut rcc);
    let _gpio_b = _dp.GPIOB.split(&mut rcc);
    let _gpio_c = _dp.GPIOC.split(&mut rcc);
    let _gpio_d = _dp.GPIOD.split(&mut rcc);
    let _gpio_e = _dp.GPIOE.split(&mut rcc);

    let onboard_button = _gpio_a.pa0.into_pull_up_input();
    let mut onboard_led = _gpio_c.pc13.into_push_pull_output();

    let mut output_pin = _gpio_a.pa1.into_push_pull_output();

    let dcc_data:[u8; 6] = [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00];

    loop {
        if onboard_button.is_low() {
            onboard_led.set_low();
            send_data(&mut output_pin, &dcc_data);
            onboard_led.set_high();
        } else {
            cortex_m::asm::nop();
        }
    }
}

const CLK_HZ: u32 = 84_000_000;
const ONE_HALF_PERIOD: u32 = 58 * (CLK_HZ / 1_000_000);
const ZERO_HALF_PERIOD: u32 = 100 * (CLK_HZ / 1_000_000);

fn send_data<P: OutputPin>(pin: &mut P, dcc_data: &[u8; 6]) {
    for _ in 0..20 {
        send_bit(pin, true);
    }

    send_bit(pin, false);

    for (i, byte) in dcc_data.iter().enumerate() {
        for bit in (0..8).rev() {
            let b = (byte >> bit) & 1 == 1;
            send_bit(pin, b);
        }

        if i == dcc_data.len() - 1 {
            send_bit(pin, true);
        } else {
            send_bit(pin, false);
        }
    }
}

fn send_bit<P: OutputPin>(pin: &mut P, bit: bool) {
    let cycles = if bit { ONE_HALF_PERIOD } else { ZERO_HALF_PERIOD };

    pin.set_high().ok();
    delay(cycles);

    pin.set_low().ok();
    delay(cycles);
}