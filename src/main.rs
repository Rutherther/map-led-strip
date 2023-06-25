#![no_std]
#![no_main]

mod strip;
mod map;
mod commands;

use embedded_hal::serial::{Write};
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::{TimerGroup}, Rtc, IO, Delay, PulseControl, Uart};
use hal::uart::config::{Config, DataBits, Parity, StopBits};
use hal::uart::TxRxPins;
use nb::block;
use nb::Error::{Other};
use smart_leds::{RGB8, SmartLedsWrite};
use crate::commands::all_command::AllCommand;
use crate::commands::command_handler::{CommandHandler};
use crate::commands::command_handler;
use crate::commands::hello_world_command::HelloWorldCommand;
use crate::commands::reset_command::ResetCommand;
use crate::commands::set_command::SetCommand;
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
    let mut map = map::Map::new(&map::INDEX_MAP, &mut rgb_data);

    let world_command = HelloWorldCommand::default();
    let set_command = SetCommand::default();
    let reset_command = ResetCommand::default();
    let all_command = AllCommand::default();
    let mut handler = CommandHandler::new(
        [
            ("HELLO_WORLD", &world_command),
            ("SET", &set_command),
            ("RESET", &reset_command),
            ("ALL", &all_command)
        ],
        ['\0'; COMMAND_BUFFER],
    );

    block!(serial.write(b'>')).ok().unwrap();
    block!(serial.write(b' ')).ok().unwrap();
    loop {
        let handled = match handler.read_command(&mut serial) {
            Ok(()) => {
                println!("\r");
                let result = handler.handle_command(&mut map);

                match result {
                    Ok(()) => Ok(true),
                    Err(err) => Err(err)
                }
            }
            Err(err) => {
                match err {
                    Other(error) => {
                        match error {
                            command_handler::CommandReadError::BufferOverflowed => println!("Command is too long.\r"),
                            command_handler::CommandReadError::UnexpectedEndOfLine => (),
                            command_handler::CommandReadError::CommandLoadedAlready => println!("FATAL: Previous command not processed correctly.\r")
                        };
                        Ok(true)
                    }
                    _ => Ok(false)
                }
            }
        };

        let new_command = match handled {
            Ok(handled) => {
                handled
            }
            Err(err) => {
                match err {
                    command_handler::CommandHandleError::NotFound => println!("Command not found.\r"),
                    command_handler::CommandHandleError::WrongArguments => println!("Wrong arguments.\r"),
                    command_handler::CommandHandleError::CommandNotRead => println!("FATAL: Command is not prepared.\r")
                }
                true
            }
        };

        if new_command {
            println!("\r");
            block!(serial.write(b'>')).ok().unwrap();
            block!(serial.write(b' ')).ok().unwrap();
        }

        strip.write(map.get_map().cloned()).unwrap();
        delay.delay_us(500u32);
    }
}