use alloc::boxed::Box;
use crate::animations::animation::{Animation, AnimationError};
use crate::animations::animation_step::AnimationStep;
use crate::map::Map;

pub struct AnimationStorage {
    animation: Option<Box<dyn Animation>>,
}

impl AnimationStorage {
    pub fn new() -> Self {
        Self {
            animation: None
        }
    }

    pub fn animation<'a>(&'a mut self) -> Option<impl Animation + 'a> {
        if self.animation.is_none() {
            None
        } else {
            Some(StorageAnimation { storage: self })
        }
    }

    pub fn set_animation<T: Animation + 'static>(&mut self, animation: T) -> () {
        self.animation = Some(Box::new(animation));
    }

    pub fn remove_animation(&mut self) -> () {
        self.animation = None;
    }
}

struct StorageAnimation<'a> {
    storage: &'a mut AnimationStorage
}

impl<'a> Animation for StorageAnimation<'a> {
    fn is_started(&self) -> bool {
        self.storage.animation.as_ref().unwrap().is_started()
    }

    fn init(&mut self) -> Result<(), AnimationError> {
        self.storage.animation.as_mut().unwrap().init()
    }

    fn next(&mut self) -> Result<AnimationStep, AnimationError> {
        self.storage.animation.as_mut().unwrap().next()
    }

    fn apply(&mut self, map: &mut Map) -> Result<(), AnimationError> {
        self.storage.animation.as_mut().unwrap().apply(map)
    }
}