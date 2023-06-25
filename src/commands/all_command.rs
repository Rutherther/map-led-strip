use esp_println::println;
use crate::commands::command_handler::{CommandHandleError, SpecificCommandHandler};
use crate::commands::command_handler::CommandHandleError::WrongArguments;
use crate::commands::command_data::CommandData;

#[derive(Default)]
pub struct AllCommand;

impl SpecificCommandHandler for AllCommand {
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError> {
        let (cmd, map) = command.deconstruct();

        if cmd.parsed_arguments().len() < 4 {
            println!("Less than 4 args.");
            return Err(WrongArguments);
        }

        let r = cmd.parsed_arguments()[1].try_to_integer();
        let g = cmd.parsed_arguments()[2].try_to_integer();
        let b = cmd.parsed_arguments()[3].try_to_integer();

        if r.is_none() || g.is_none() || b.is_none() {
            println!("Cold not parse r, g, b.");
            return Err(WrongArguments);
        }

        let r = r.unwrap();
        let g = g.unwrap();
        let b = b.unwrap();

        if r > 255 || g > 255 || b > 255 {
            return Err(WrongArguments);
        }

        for led in map.get_map_mut() {
            led.r = r as u8;
            led.g = g as u8;
            led.b = b as u8;
        }

        Ok(())
    }

    fn help(&self) -> &'static str {
        "<R> <G> <B> - light up all LEDs to the given color"
    }
}