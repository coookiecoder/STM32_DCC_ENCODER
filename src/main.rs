#![no_std]
#![no_main]

#![allow(unused_imports)]

pub mod utils;

use panic_halt;

use stm32f4xx_hal as hal;

use hal::pac;

use hal::prelude::_stm32f4xx_hal_rcc_RccExt;
use hal::prelude::_stm32f4xx_hal_gpio_GpioExt;
use hal::prelude::_fugit_RateExtU32;

use hal::rcc::Config;

use hal::spi::SpiSlave;
use hal::spi::Mode;
use hal::spi::Phase;
use hal::spi::Polarity;

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
const DATA_SIZE: usize = 64;

use utils::send_reset;
use utils::send_idle;
use utils::send_stop;
use utils::send_data;

#[entry]
fn main() -> ! {
    let _dp = pac::Peripherals::take().expect("cannot take peripherals");
    let mut _cp = pac::CorePeripherals::take().expect("cannot take core peripherals");

    let config = Config::hse(25.MHz()).sysclk(84.MHz()).require_pll48clk();
    let mut rcc = _dp.RCC.freeze(config);

    _cp.DCB.enable_trace();
    _cp.DWT.enable_cycle_counter();

    let _gpio_a = _dp.GPIOA.split(&mut rcc);
    let _gpio_b = _dp.GPIOB.split(&mut rcc);
    let _gpio_c = _dp.GPIOC.split(&mut rcc);
    let _gpio_d = _dp.GPIOD.split(&mut rcc);
    let _gpio_e = _dp.GPIOE.split(&mut rcc);

    let mut dcc_output = _gpio_a.pa5.into_push_pull_output();

    let panic_input = _gpio_a.pa0.into_input();
    let stop_input = _gpio_a.pa1.into_input();

    let mut data: [u8; DATA_SIZE] = [0u8; DATA_SIZE];
    let mut last_data: [u8; DATA_SIZE] = [0u8; DATA_SIZE];

    loop {
        if panic_input.is_low() {
            send_reset(&mut dcc_output, &_cp.DWT);

            data = [0u8; DATA_SIZE];

            continue;
        } else if stop_input.is_low() {
            send_stop(&mut dcc_output, &_cp.DWT, true);

            continue;
        }

        if data != last_data {
            send_data(&mut dcc_output, &_cp.DWT, &data);
        } else {
            send_idle(&mut dcc_output, &_cp.DWT);
        }

        last_data = data;
    }
}