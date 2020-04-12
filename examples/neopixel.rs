#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use hal::delay::Delay;
use hal::timer::Timer;
use hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

use feather_f405::{clock_setup, NeoPixel};
use smart_leds::{SmartLedsWrite, RGB8};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let clocks = clock_setup(dp.RCC);

    let mut delay = Delay::new(p.SYST, clocks);
    let gpioc = dp.GPIOC.split();

    let timer = Timer::tim2(dp.TIM2, 3.mhz(), clocks);
    let mut neopixel = NeoPixel::new(gpioc.pc0, timer);

    let mut data = RGB8 { r: 0, g: 0, b: 0 };

    loop {
        data.r = data.r.wrapping_add(4);
        data.g = data.g.wrapping_add(8);
        data.b = data.b.wrapping_add(16);

        neopixel.ws.write([data].iter().cloned()).unwrap();

        delay.delay_ms(50u16);
    }
}
