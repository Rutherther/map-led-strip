use crate::animations::animation_storage::AnimationStorage;
use crate::commands::command::Command;
use crate::map::Map;

pub struct CommandData<'d, 'a> {
    command: &'d Command<'d>,
    map: &'d mut Map<'a>,
    animation_storage: &'d mut AnimationStorage
}

impl<'d, 'a> CommandData<'d, 'a> {
    pub fn new(command: &'d Command<'d>, map: &'d mut Map<'a>, animation_manager: &'d mut AnimationStorage) -> Self {
        CommandData {
            command,
            map,
            animation_storage: animation_manager
        }
    }

    pub fn animation_storage(self) -> &'d mut AnimationStorage {
        self.animation_storage
    }

    pub fn command(self) -> &'d Command<'d> {
        self.command
    }

    pub fn map(self) -> &'d mut Map<'a> {
        self.map
    }

    pub fn deconstruct_map(self) -> (&'d Command<'d>, &'d mut Map<'a>) {
        (self.command, self.map)
    }

    pub fn deconstruct_animation(self) -> (&'d Command<'d>, &'d mut AnimationStorage) {
        (self.command, self.animation_storage)
    }

    pub fn deconstruct(self) -> (&'d Command<'d>, &'d mut Map<'a>, &'d mut AnimationStorage) {
        (self.command, self.map, self.animation_storage)
    }
}