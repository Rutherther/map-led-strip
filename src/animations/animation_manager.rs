use embedded_hal::timer::CountDown;
use fugit::MicrosDurationU64;
use nb::Error::{Other, WouldBlock};
use crate::animations::animation::{Animation, AnimationError};
use crate::animations::animation_storage::AnimationStorage;
use crate::map::Map;

pub struct AnimationManager<Timer> {
    timer: Timer,
    storage: AnimationStorage,
}

impl<Timer: CountDown<Time=MicrosDurationU64>> AnimationManager<Timer> {
    pub fn new(timer: Timer) -> Self {
        Self {
            timer,
            storage: AnimationStorage::new(),
        }
    }

    pub fn storage(&mut self) -> &mut AnimationStorage {
        &mut self.storage
    }

    pub fn update(&mut self, map: &mut Map) -> nb::Result<(), AnimationError> {
        let step_result;
        {
            let mut animation = if let Some(animation) = self.storage.animation() {
                animation
            } else { // animation is not set, no job to do
                return Ok(());
            };

            let mut timer_ran_off = false;
            if animation.is_started() {
                timer_ran_off = match self.timer.wait() {
                    Err(_) => return Err(WouldBlock),
                    _ => true // timer has reached next step
                };
            }

            step_result = if !animation.is_started() || timer_ran_off {
                animation.next()
            } else {
                return Err(WouldBlock);
            };
            animation.apply(map)?;
        }

        let step = match step_result {
            Ok(step) => step,
            Err(err) if err == AnimationError::LastStep => {
                self.storage.remove_animation();
                return Ok(());
            }
            Err(err) => {
                return Err(Other(err));
            }
        };

        self.timer.start(step.duration());
        Ok(())
    }
}