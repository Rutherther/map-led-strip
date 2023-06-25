use crate::commands::command::Command;
use crate::map::Map;

pub struct CommandData<'d, 'a> {
    command: &'d Command<'d>,
    map: &'d mut Map<'a>,
}

impl<'d, 'a> CommandData<'d, 'a> {
    pub fn new(command: &'d Command<'d>, map: &'d mut Map<'a>) -> Self {
        CommandData {
            command,
            map
        }
    }

    pub fn command(self) -> &'d Command<'d> {
        self.command
    }

    pub fn map(self) -> &'d mut Map<'a> {
        self.map
    }

    pub fn deconstruct(self) -> (&'d Command<'d>, &'d mut Map<'a>) {
        (self.command, self.map)
    }
}