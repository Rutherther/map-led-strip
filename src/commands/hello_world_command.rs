use esp_println::{print, println};
use crate::command_handler::{CommandHandleError, SpecificCommandHandler};
use crate::commands::command_data::CommandData;

#[derive(Default)]
pub struct HelloWorldCommand;

impl SpecificCommandHandler for HelloWorldCommand
{
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError>
    {
        let command = command.command();
        let full = command.full();
        print!("Hello world!");
        for c in &full[command.parsed_arguments()[0].data.len()..] {
            print!("{}", c);
        }
        println!("\r");

        Ok(())
    }

    fn help(&self) -> &'static str {
        "<x> - prints Hello world! <x>"
    }
}