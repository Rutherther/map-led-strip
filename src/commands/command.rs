use crate::commands::command_argument::CommandArgument;

pub struct Command<'d>
{
    pub full: &'d [char],
    pub parsed_arguments: &'d [CommandArgument<'d>],
}