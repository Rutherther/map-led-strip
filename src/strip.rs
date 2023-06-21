use core::slice::IterMut;
use hal::gpio::{OutputPin};
use hal::peripheral::Peripheral;
use hal::pulse_control::{OutputChannel, ConfiguredChannel, PulseCode, RepeatMode, TransmissionError, ClockSource};
use smart_leds::{RGB8, SmartLedsWrite};
use fugit::{Duration, NanosDuration};

pub struct Strip<CHANNEL, const COUNT: usize> {
    channel: CHANNEL,
    timing: StripTiming
}

pub struct StripTiming {
    one_high_duration: Duration::<u32, 1, 1_000_000_000>,
    one_low_duration: Duration::<u32, 1, 1_000_000_000>,
    zero_high_duration: Duration::<u32, 1, 1_000_000_000>,
    zero_low_duration: Duration::<u32, 1, 1_000_000_000>,
}

impl StripTiming {
    pub fn new(one_high_duration: Duration::<u32, 1, 1_000_000_000>,
               one_low_duration: Duration::<u32, 1, 1_000_000_000>,
               zero_high_duration: Duration::<u32, 1, 1_000_000_000>,
               zero_low_duration: Duration::<u32, 1, 1_000_000_000>) -> Self {
        StripTiming {
            one_high_duration,
            one_low_duration,
            zero_high_duration,
            zero_low_duration,
        }
    }
}

impl<'d, CHANNEL, const COUNT: usize> Strip<CHANNEL, COUNT>
    where CHANNEL: ConfiguredChannel
{
    pub fn new<P: OutputPin + 'd, UnconfiguredChannel>(mut channel: UnconfiguredChannel, pin: impl Peripheral<P=P> + 'd, timing: StripTiming) -> Self
        where UnconfiguredChannel: OutputChannel<ConfiguredChannel<'d, P> = CHANNEL>
    {
        channel
            .set_channel_divider(4) // 1 tick = 50 ns = 0.05 us
            .set_carrier_modulation(false)
            .set_idle_output(true)
            .set_idle_output_level(false)
            .set_clock_source(ClockSource::APB);

        let channel = channel.assign_pin(pin);

        Strip::<CHANNEL, COUNT> {
            channel,
            timing
        }
    }

    fn byte_to_pulse_code(&self, byte: u8, data: &mut IterMut<u32>) -> () {
        for i in 0..8u8 {
            let bit = if (byte & (1 << (7 - i))) > 0 { true } else { false };
            *data.next().unwrap() = self.bit_to_pulse_code(bit);
        }
    }

    fn bit_to_pulse_code(&self, bit: bool) -> u32 {
        let length1 = if bit { self.timing.one_high_duration } else { self.timing.zero_high_duration };
        let length2 = if bit { self.timing.one_low_duration } else { self.timing.zero_low_duration };

        let length1 = NanosDuration::<u32>::from_ticks(length1.ticks() / 50);
        let length2 = NanosDuration::<u32>::from_ticks(length2.ticks() / 50);

        PulseCode {
            level1: true,
            length1,
            level2: false,
            length2,
        }.into()
    }
}

impl<CHANNEL, const COUNT: usize> SmartLedsWrite for Strip<CHANNEL, COUNT>
    where CHANNEL: ConfiguredChannel
{
    type Error = TransmissionError;
    type Color = RGB8;

    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error> where T: Iterator<Item=I>, I: Into<Self::Color> {
        let mut buffer: [u32; COUNT] = [0; COUNT];
        let mut iter_mut = buffer.iter_mut();
        for item in iterator {
            let rgb = item.into();
            self.byte_to_pulse_code(rgb.g, &mut iter_mut);
            self.byte_to_pulse_code(rgb.r, &mut iter_mut);
            self.byte_to_pulse_code(rgb.b, &mut iter_mut);
        }
        *iter_mut.next().unwrap() = 0;

        self.channel.send_pulse_sequence_raw(RepeatMode::SingleShot, &buffer)
    }
}