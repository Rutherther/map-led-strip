use embedded_hal::serial::{Read, Write};
use esp_println::println;
use nb::block;
use nb::Error::{Other, WouldBlock};
use crate::command_handler::{CommandHandleError::{CommandNotRead, NotFound}, CommandReadError::{BufferOverflowed, CommandLoadedAlready, UnexpectedEndOfLine}};
use crate::commands::{command::Command, command_argument::CommandArgument, command_data::CommandData};
use crate::map::Map;

pub trait SpecificCommandHandler {
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError>;
    fn help(&self) -> &'static str;
}

pub struct CommandHandler<'d, const BUFFER_SIZE: usize, const HANDLERS_COUNT: usize> {
    buffer_position: usize,
    command_loaded: bool,
    buffer: [char; BUFFER_SIZE],
    handlers: [(&'d str, &'d dyn SpecificCommandHandler); HANDLERS_COUNT],
    handling_special: u8
}

#[derive(Debug, Eq, PartialEq)]
pub enum CommandReadError {
    UnexpectedEndOfLine,
    BufferOverflowed,
    CommandLoadedAlready,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CommandHandleError {
    NotFound,
    WrongArguments,
    CommandNotRead,
}

impl<'d, const BUFFER_SIZE: usize, const HANDLERS_COUNT: usize> CommandHandler<'d, BUFFER_SIZE, HANDLERS_COUNT> {
    pub fn new(handlers: [(&'d str, &'d dyn SpecificCommandHandler); HANDLERS_COUNT], buffer: [char; BUFFER_SIZE]) -> Self
    {
        Self {
            command_loaded: false,
            handling_special: 0,
            buffer_position: 0,
            buffer,
            handlers,
        }
    }

    pub fn reset(&mut self) -> ()
    {
        self.buffer_position = 0;
        self.command_loaded = false;
        self.handling_special = 0;
    }

    fn handle_special<Serial>(&mut self, serial: &mut Serial, data: u8) -> bool
        where Serial: Read<u8> + Write<u8>
    {
        if self.handling_special > 0 { // some special characters have char length 2.
            self.handling_special -= 1;
            return false;
        }

        match data {
            b'\x08' => { // backspace
                if self.buffer_position > 0 {
                    self.buffer_position -= 1;
                    block!(serial.write(b'\x08')).ok().unwrap();
                    block!(serial.write(b' ')).ok().unwrap();
                    block!(serial.write(b'\x08')).ok().unwrap();
                }
                false
            },
            b'\x1b' => { // other special characters
                self.handling_special = 2;
                false
            },
            _ => true
        }
    }

    pub fn read_command<Serial>(&mut self, serial: &mut Serial) -> nb::Result<(), CommandReadError>
        where Serial: Read<u8> + Write<u8>
    {
        if self.command_loaded {
            return Err(Other(CommandLoadedAlready));
        }

        let read = serial.read();

        if self.buffer_position >= BUFFER_SIZE {
            return Err(Other(BufferOverflowed));
        }

        match read {
            Ok(data) => {
                if self.handle_special(serial, data) {
                    self.buffer[self.buffer_position] = data as char
                } else { // special character handled
                    return Err(WouldBlock);
                }
            }
            Err(_) => return Err(WouldBlock)
        };

        let read = self.buffer[self.buffer_position];
        self.buffer_position += 1;

        block!(serial.write(read as u8)).ok().unwrap();

        if read != '\r' {
            return Err(WouldBlock);
        }

        block!(serial.write(b'\n')).ok().unwrap();

        if self.buffer_position <= 1 {
            self.reset();
            return Err(Other(UnexpectedEndOfLine));
        }

        self.command_loaded = true;
        Ok(())
    }

    fn parse_command<'a>(&self, buffer: &'a [char], args: &'a mut [CommandArgument<'a>]) -> Command<'a>
    {
        let mut last_arg_index = 0;
        let mut length = 0;
        let mut args_iter = args.iter_mut();

        for i in 0..buffer.len() {
            let argument_length = i - last_arg_index;

            if buffer[i] == ' ' {
                if argument_length > 0 {
                    *args_iter.next().unwrap() = CommandArgument::new(&buffer[last_arg_index..i]);
                    length += 1;
                }

                last_arg_index = i + 1;
            }

            if buffer[i] == '\r' || buffer[i] == '\n' {
                if argument_length > 0 {
                    *args_iter.next().unwrap() = CommandArgument::new(&buffer[last_arg_index..i]);
                    length += 1;
                }

                break;
            }
        }

        Command::new(buffer, &args[0..length])
    }

    fn handle_help(&self) -> Result<(), CommandHandleError>
    {
        println!("Available commands:\r");
        for (cmd, handler) in self.handlers {
            println!("  {0} {1}\r", cmd, handler.help());
        }

        Ok(())
    }

    pub fn handle_command(&mut self, map: &mut Map) -> Result<(), CommandHandleError>
    {
        if !self.command_loaded {
            return Err(CommandNotRead);
        }

        let buffer = &self.buffer[0..self.buffer_position];
        let mut args = [CommandArgument::new(buffer); BUFFER_SIZE];

        let command = self.parse_command(buffer, &mut args);

        if command.parsed_arguments().len() == 0 {
            self.reset();
            return Err(NotFound);
        }

        let first_argument = command.parsed_arguments()[0];

        if first_argument.compare("HELP") {
            let help_handled = self.handle_help();
            self.reset();
            return help_handled;
        }

        for (handler_command, handler) in self.handlers {
            if !first_argument.compare(handler_command) {
                continue;
            }

            let command_data = CommandData::new(&command, map);
            let handled = handler.handle(command_data);
            self.reset();
            return handled;
        }

        self.reset();
        Err(NotFound)
    }
}

// HELP
// DEFAULT <R> <G> <B>
// SET <X:id or name> <R> <G> <B>
// ALL <R> <G> <B>
// RESET
// ITER [X] [Y] (X start, Y end)
// SNAKE [X] [Y] (X start, Y end)
// RAINBOW -- show rainbow

// WIFI <SSID> <PASS>
