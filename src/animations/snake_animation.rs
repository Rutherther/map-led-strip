use core::cmp::max;
use esp_println::println;
use fugit::MicrosDurationU64;
use libm::{ceilf, powf};
use smart_leds::RGB8;
use crate::animations::animation::{Animation, AnimationError};
use crate::animations::animation_step::AnimationStep;
use crate::map::Map;

pub struct SnakeAnimation<const LEDS_COUNT: usize> {
    step: usize,
    finished: bool,
    order: [usize; LEDS_COUNT],
    previous_factor: f32,
    color: RGB8,
    step_duration: MicrosDurationU64,
}

impl<const LEDS_COUNT: usize> SnakeAnimation<LEDS_COUNT> {
    pub fn new(order: [usize; LEDS_COUNT], previous_factor: f32, color: RGB8, step_duration: MicrosDurationU64) -> Self {
        Self {
            step: 0,
            finished: false,
            order,
            previous_factor,
            color,
            step_duration
        }
    }

    fn is_first_step(&self) -> bool {
        self.step == 1
    }

    fn is_last_step(&self) -> bool {
        self.finished
    }
}

impl<const LEDS_COUNT: usize> Animation for SnakeAnimation<LEDS_COUNT> {
    fn is_started(&self) -> bool {
        self.step > 0
    }

    fn init(&mut self) -> Result<(), AnimationError> {
        Ok(())
    }

    fn next(&mut self) -> Result<AnimationStep, AnimationError> {
        if self.step == LEDS_COUNT + 100 {
            self.finished = true;
            return Err(AnimationError::LastStep);
        }

        self.step += 1;
        Ok(AnimationStep::new(self.step_duration))
    }

    fn apply(&mut self, map: &mut Map) -> Result<(), AnimationError> {
        if self.is_first_step() {
            map.clear();
        }

        for (i, led_index) in self.order.iter().take(self.step.max(LEDS_COUNT)).enumerate() {
            let mult_factor = self.step - i - 1;
            let coeff = powf(self.previous_factor, mult_factor as f32);
            let rgb = self.color;
            let color = RGB8 {
                r: ceilf(rgb.r as f32 * coeff) as u8,
                g: ceilf(rgb.g as f32 * coeff) as u8,
                b: ceilf(rgb.r as f32 * coeff) as u8,
            };

            map.set(*led_index, color).ok().unwrap();
        }

        if self.is_last_step() {
            map.clear();
        }

        Ok(())
    }
}