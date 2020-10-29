#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::blocking::delay::DelayMs;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{
        p0::{
            Parts as P0Parts
        }, 
        Input, Level, Output, PushPull, Pin, PullUp
    },
    pac::TIMER0,
    prelude::*,
    Temp, 
    timer::OneShot,
    Timer,
    twim,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: initialize temperature sensor
    let mut temp = Temp::new(board.TEMP);

    let pins = P0Parts::new(board.P0);

    // instanciate I2C
    let scl = pins.p0_03.degrade();
    let sda = pins.p0_04.degrade();

    let pins = twim::Pins { scl, sda };
    let mut i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    loop {
        let temperature: f32 = temp.measure().to_num();
        defmt::info!("{:?} °C", temperature);
        timer.delay_ms(1000_u32)
    }

    // knurling_session_20q4::exit()
}
