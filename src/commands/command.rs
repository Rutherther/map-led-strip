use crate::commands::command_argument::CommandArgument;

pub struct Command<'d>
{
    full: &'d [char],
    parsed_arguments: &'d [CommandArgument<'d>],
}

impl<'d> Command<'d> {
    pub fn new(full: &'d [char], parsed_arguments: &'d [CommandArgument<'d>]) -> Self {
        Self {
            full,
            parsed_arguments
        }
    }
    pub fn full(&self) -> &'d [char] {
        self.full
    }

    pub fn parsed_arguments(&self) -> &'d [CommandArgument<'d>] {
        self.parsed_arguments
    }
}