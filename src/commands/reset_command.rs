use crate::commands::command_handler::{CommandHandleError, SpecificCommandHandler};
use crate::commands::command_data::CommandData;

#[derive(Default)]
pub struct ResetCommand;

impl SpecificCommandHandler for ResetCommand {
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError> {
        for led in command.map().get_map_mut() {
            led.r = 0;
            led.g = 0;
            led.b = 0;
        }
        Ok(())
    }

    fn help(&self) -> &'static str {
        "- Resets the board, all leds set to 0, 0, 0"
    }
}