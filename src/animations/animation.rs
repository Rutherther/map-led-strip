// different kinds of animation
// steps, each step has given length
// each step has LED states

// each step will show current led states on the board

use libm::{ceilf, powf};
use crate::animations::animation_step::AnimationStep;
use crate::map::Map;

pub trait Animation {
    fn is_started(&self) -> bool;
    fn init(&mut self) -> Result<(), AnimationError>;
    fn next(&mut self) -> Result<AnimationStep, AnimationError>;
    fn apply(&mut self, map: &mut Map) -> Result<(), AnimationError>;
}

#[derive(Eq, PartialEq)]
pub enum AnimationError {
    LastStep
}