use crate::commands::command_handler::{CommandHandleError, SpecificCommandHandler};
use crate::commands::command_data::CommandData;

#[derive(Default)]
pub struct ResetCommand;

impl SpecificCommandHandler for ResetCommand {
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError> {
        let (_, map, animation) = command.deconstruct();
        map.clear();
        animation.remove_animation();
        Ok(())
    }

    fn help(&self) -> &'static str {
        "- Resets the board, all leds set to 0, 0, 0"
    }
}