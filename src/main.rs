#![no_std]
#![no_main]

mod strip;
use embedded_hal::serial::{Read, Write};
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, pulse_control::{ClockSource}, peripherals::Peripherals, prelude::*, timer::{TimerGroup, Timer, Timer0}, Rtc, IO, Delay, interrupt, PulseControl, Uart};
use hal::uart::config::{Config, DataBits, Parity, StopBits};
use hal::uart::TxRxPins;
use nb::block;
use nb::Error::{WouldBlock, Other};
use smart_leds::{RGB8, SmartLedsWrite};
use crate::strip::StripTiming;
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

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio1.into_push_pull_output(),
        io.pins.gpio3.into_floating_input(),
    );

    let config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    let mut serial = Uart::new_with_config(
        peripherals.UART0,
        Some(config),
        Some(pins),
        &clocks,
        &mut system.peripheral_clock_control,
    );

    let mut delay = Delay::new(&clocks);

    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
    ).unwrap();

    let mut strip = strip::Strip::<_, { LEDS_COUNT * 24 + 1 }>::new(
        pulse.channel0,
        io.pins.gpio25,
        StripTiming::new(
            800u32.nanos(),
            450u32.nanos(),
            400u32.nanos(),
            850u32.nanos(),
        ),
    );

    let mut rgb_data: [RGB8; 72] = [RGB8 { r: 0, g: 0, b: 0 }; 72];
    loop {

    }
}