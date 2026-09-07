use crate::DATA_SIZE;

use stm32f4xx_hal as hal;

use hal::hal_02::digital::v2::OutputPin;

use cortex_m::peripheral::DWT;

const ONE: u32 = 58;
const ZERO: u32 = 100;

pub fn delay_us(dwt: &DWT, time_us: u32) {
    let start_cycles = dwt.cyccnt.read();

    let until = time_us.wrapping_mul(84);

    while dwt.cyccnt.read().wrapping_sub(start_cycles) < until {
        cortex_m::asm::nop();
    }
}

pub fn send_data<P: OutputPin> (pin: &mut P, dwt: &DWT, data: &[u8; DATA_SIZE]) {
    for _preamble in 0..14 {
        send_one(pin, dwt);
    }

    for byte in data {
        send_zero(pin, dwt);

        for bit in (0..8).rev() {
            if byte & (1 << bit) != 0 {
                send_one(pin, dwt);
            } else {
                send_zero(pin, dwt);
            }
        }
    }

    send_one(pin, dwt);
}

pub fn send_reset<P: OutputPin> (pin: &mut P, dwt: &DWT) {
    for _preamble in 0..14 {
        send_one(pin, dwt);
    }

    for _byte in 0..3 {
        send_zero(pin, dwt);

        for _bit in 0..8 {
            send_zero(pin, dwt);
        }
    }

    send_one(pin, dwt);
}

pub fn send_idle<P: OutputPin> (pin: &mut P, dwt: &DWT) {
    for _preamble in 0..14 {
        send_one(pin, dwt);
    }

    send_zero(pin, dwt);

    for _bit in 0..8 {
        send_one(pin, dwt);
    }

    send_zero(pin, dwt);

    for _bit in 0..8 {
        send_zero(pin, dwt);
    }

    send_zero(pin, dwt);

    for _bit in 0..8 {
        send_one(pin, dwt);
    }

    send_one(pin, dwt);
}

pub fn send_stop<P: OutputPin> (pin: &mut P, dwt: &DWT, fast: bool) {
    for _preamble in 0..14 {
        send_one(pin, dwt);
    }

    send_zero(pin, dwt);

    for _bit in 0..8 {
        send_zero(pin, dwt);
    }

    for _byte in 0..2 {
        send_zero(pin, dwt);

        {
            send_zero(pin, dwt);
            send_one(pin, dwt);
            send_zero(pin, dwt);
            send_zero(pin, dwt);
            send_zero(pin, dwt);
            send_zero(pin, dwt);
            send_zero(pin, dwt);
            if fast {
                send_one(pin, dwt);
            } else {
                send_zero(pin, dwt);
            }
        }
    }

    send_one(pin, dwt);
}

pub fn send_one<P: OutputPin> (pin: &mut P, dwt: &DWT) {
    pin.set_low().ok();
    delay_us(dwt, ONE);
    pin.set_high().ok();
    delay_us(dwt, ONE);
}

pub fn send_zero<P: OutputPin> (pin: &mut P, dwt: &DWT) {
    pin.set_low().ok();
    delay_us(dwt, ZERO);
    pin.set_high().ok();
    delay_us(dwt, ZERO);
}