use crate::commands::command_handler::{CommandData, CommandHandleError, SpecificCommandHandler};
use crate::commands::command_handler::CommandHandleError::WrongArguments;

pub struct ResetCommand {}

impl ResetCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecificCommandHandler for ResetCommand {
    fn handle(&self, command: &mut CommandData) -> Result<(), CommandHandleError> {
        for led in command.map.get_map_mut() {
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