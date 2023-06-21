use embedded_hal::serial::{Read, Write};
use esp_println::{print, println};
use crate::command_handler::{CommandData, CommandHandleError, SpecificCommandHandler};
use crate::map::Map;

pub struct HelloWorldCommand {

}

impl HelloWorldCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecificCommandHandler for HelloWorldCommand
{
    fn handle(&self, command: &mut CommandData) -> Result<(), CommandHandleError>
    {
        print!("Hello world!");
        for c in &command.command.full[command.command.parsed_arguments[0].data.len()..] {
            print!("{}", c);
        }
        println!("\r");

        Ok(())
    }

    fn help(&self) -> &'static str {
        "<x> - prints Hello world! <x>"
    }
}