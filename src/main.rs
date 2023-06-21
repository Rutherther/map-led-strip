#![no_std]
#![no_main]

use embedded_hal::serial::{Read, Write};
use esp_backtrace as _;
use hal::{clock::ClockControl, pulse_control::{ClockSource}, peripherals::Peripherals, prelude::*, timer::{TimerGroup, Timer, Timer0}, Rtc, IO, Delay, interrupt, PulseControl, Uart};
use hal::uart::config::{Config, DataBits, Parity, StopBits};
use nb::Error::{WouldBlock, Other};
const LEDS_COUNT: usize = 72;
const COMMAND_BUFFER: usize = 200;

#[entry]
fn main() -> ! {
    // setup
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    loop {

    }
}