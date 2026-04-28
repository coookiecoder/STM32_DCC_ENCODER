#![no_std]
#![no_main]

#![allow(unused_imports)]

use panic_halt;

use stm32f4xx_hal as hal;

use hal::pac;
use hal::pac::interrupt;

use hal::prelude::_stm32f4xx_hal_rcc_RccExt;
use hal::prelude::_stm32f4xx_hal_gpio_GpioExt;
use hal::prelude::_fugit_RateExtU32;

use hal::rcc::Config;

use hal::timer::Timer;
use hal::timer::TimerExt;
use hal::timer::Event;

use hal::Listen;

use cortex_m::peripheral::NVIC;

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

    let mut timer2 = _dp.TIM2.counter_hz(&mut rcc);
    timer2.start(10.kHz()).expect("Failed to start TIM2");
    timer2.listen(Event::Update);

    unsafe { NVIC::unmask(pac::Interrupt::TIM2); }

    loop {
        if onboard_button.is_low() {
            onboard_led.set_low();
        } else {
            onboard_led.set_high();
        }
    }
}

const TICKS_100US: u32 = 8400 - 1;
const TICKS_58US: u32 = 4872 - 1;

#[interrupt]
fn TIM2() {
    let tim2 = unsafe { &*pac::TIM2::ptr() };

    tim2.sr().modify(|_, w| w.uif().clear_bit());

    let should_use_long_delay = true;

    if should_use_long_delay {
        tim2.arr().write(|w| unsafe { w.bits(TICKS_100US) });
    } else {
        tim2.arr().write(|w| unsafe { w.bits(TICKS_58US) });
    }
}