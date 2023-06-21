use esp_println::println;
use crate::commands::command_handler::{CommandData, CommandHandleError, SpecificCommandHandler};
use crate::commands::command_handler::CommandHandleError::WrongArguments;

pub struct SetCommand {}

impl SetCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecificCommandHandler for SetCommand {
    fn handle(&self, command: &mut CommandData) -> Result<(), CommandHandleError> {
        let cmd = &command.command;
        let map = &mut command.map;

        if cmd.parsed_arguments.len() < 5 {
            println!("Less than 5 args.");
            return Err(WrongArguments);
        }

        let led_id = cmd.parsed_arguments[1];
        let led_id = if let Some(id) = led_id.try_to_integer() {
            Some(id as usize)
        } else {
            map.get_index_by_name(led_id.chars()).ok()
        };

        if led_id.is_none() {
            println!("Could not parse led id.");
            return Err(WrongArguments);
        }

        let r = cmd.parsed_arguments[2].try_to_integer();
        let g = cmd.parsed_arguments[3].try_to_integer();
        let b = cmd.parsed_arguments[4].try_to_integer();

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

        map.set_rgb(led_id.unwrap(), Some(r as u8), Some(g as u8), Some(b as u8)).ok().unwrap();

        Ok(())
    }

    fn help(&self) -> &'static str {
        "<id or name> <R> <G> <B> - Set the specified LED to the given color levels"
    }
}