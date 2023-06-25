#![no_std]
#![no_main]

extern crate alloc;

mod strip;
mod map;
mod commands;
mod animations;
mod constants;

use alloc::boxed::Box;
use embedded_hal::serial::{Read, Write};
use embedded_hal::timer::CountDown;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::{TimerGroup}, Rtc, IO, Delay, PulseControl, Uart};
use hal::uart::config::{Config, DataBits, Parity, StopBits};
use hal::uart::TxRxPins;
use nb::block;
use nb::Error::{Other};
use smart_leds::{RGB8, SmartLedsWrite};
use esp_alloc::EspHeap;
use crate::animations::animation_manager::AnimationManager;
use crate::commands::all_command::AllCommand;
use crate::commands::command_handler::{CommandHandler};
use crate::commands::command_handler;
use crate::commands::hello_world_command::HelloWorldCommand;
use crate::commands::reset_command::ResetCommand;
use crate::commands::set_command::SetCommand;
use crate::commands::snake_command::SnakeCommand;
use crate::map::Map;
use crate::strip::StripTiming;

#[global_allocator]
static ALLOCATOR: EspHeap = EspHeap::empty();

fn init_heap() {
    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        ALLOCATOR.init(heap_start as *mut u8, heap_end - heap_start);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    init_heap();

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

    // Init UART
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

    // Init strip
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
    ).unwrap();

    let mut strip = strip::Strip::<_, { constants::LEDS_COUNT * 24 + 1 }>::new(
        pulse.channel0,
        io.pins.gpio25,
        StripTiming::new(
            800u32.nanos(),
            450u32.nanos(),
            400u32.nanos(),
            850u32.nanos(),
        ),
    );

    // Init map
    let mut rgb_data: [RGB8; 72] = [RGB8 { r: 0, g: 0, b: 0 }; 72];
    let mut map = map::Map::new(&map::INDEX_MAP, &mut rgb_data);
    let mut animations = AnimationManager::new(timer_group0.timer0);
    let mut delay = Delay::new(&clocks);

    // Init commands
    let mut handler = CommandHandler::new(
        [
            ("HELLO_WORLD", Box::new(HelloWorldCommand::default())),
            ("SET", Box::new(SetCommand::default())),
            ("RESET", Box::new(ResetCommand::default())),
            ("ALL", Box::new(AllCommand::default())),
            ("SNAKE", Box::new(SnakeCommand::default()))
        ],
        ['\0'; constants::COMMAND_BUFFER],
    );

    print_new_command(&mut serial);

    loop {
        // result is either ok, then do nothing,
        // would block, then do nothing
        // or last step, then do nothing as well...
        let _ = animations.update(&mut map);

        let new_command = match handler.read_command(&mut serial) {
            Ok(()) => {
                println!("\r");
                let result = handler.handle_command(&mut map, animations.storage());

                match result {
                    Ok(()) => true,
                    Err(err) => {
                        match err {
                            command_handler::CommandHandleError::NotFound => println!("Command not found.\r"),
                            command_handler::CommandHandleError::WrongArguments => println!("Wrong arguments.\r"),
                            command_handler::CommandHandleError::CommandNotRead => println!("FATAL: Command is not prepared.\r")
                        }
                        true
                    }
                }
            },
            Err(err) => {
                match err {
                    Other(error) => {
                        match error {
                            command_handler::CommandReadError::BufferOverflowed => println!("Command is too long.\r"),
                            command_handler::CommandReadError::UnexpectedEndOfLine => (),
                            command_handler::CommandReadError::CommandLoadedAlready => println!("FATAL: Previous command not processed correctly.\r")
                        };
                        true
                    }
                    _ => false
                }
            }
        };

        if new_command {
            print_new_command(&mut serial);
        }

        strip.write(map.get_map().cloned()).unwrap();
        delay.delay_us(500u32);
    }

    fn print_new_command<T: Write<u8>>(serial: &mut T) {
        println!("\r");
        block!(serial.write(b'>')).ok().unwrap();
        block!(serial.write(b' ')).ok().unwrap();
    }
}