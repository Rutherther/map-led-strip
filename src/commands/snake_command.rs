use esp_println::println;
use fugit::ExtU64;
use smart_leds::RGB8;
use crate::animations::snake_animation::SnakeAnimation;
use crate::commands::command_data::CommandData;
use crate::commands::command_handler::{CommandHandleError, SpecificCommandHandler};
use crate::commands::command_handler::CommandHandleError::WrongArguments;
use crate::constants;

#[derive(Default)]
pub struct SnakeCommand;

impl SpecificCommandHandler for SnakeCommand {
    fn handle(&self, command: CommandData) -> Result<(), CommandHandleError> {
        let (cmd, animation) = command.deconstruct_animation();

        if cmd.parsed_arguments().len() < 6 {
            println!("Less than 6 args.");
            return Err(WrongArguments);
        }

        let coeff = cmd.parsed_arguments()[1].try_to_integer();

        let r = cmd.parsed_arguments()[2].try_to_integer();
        let g = cmd.parsed_arguments()[3].try_to_integer();
        let b = cmd.parsed_arguments()[4].try_to_integer();
        let duration = cmd.parsed_arguments()[5].try_to_integer();

        if r.is_none() || g.is_none() || b.is_none() || coeff.is_none() || duration.is_none() {
            println!("Cold not parse r, g, b, coeff or duration.");
            return Err(WrongArguments);
        }

        let coeff = coeff.unwrap();
        let r = r.unwrap();
        let g = g.unwrap();
        let b = b.unwrap();
        let duration = duration.unwrap();

        if r > 255 || g > 255 || b > 255 || coeff > 255 {
            return Err(WrongArguments);
        }
        let coeff = coeff as f32 / 255.0;

        animation
            .set_animation(SnakeAnimation::<{ constants::LEDS_COUNT }>::new(
                [24, 19, 16, 10, 15, 9, 6, 8, 3, 0, 4, 1, 2, 5, 11, 7, 12, 18, 21, 29, 27, 34, 38, 31, 17, 25, 45, 30, 36, 35, 42, 44, 56, 48, 61, 57, 65, 68, 59, 49, 55, 63, 71, 69, 62, 47, 53, 46, 51, 64, 52, 67, 70, 66, 60, 58, 54, 50, 37, 40, 41, 23, 22, 14, 13, 20, 28, 33, 39, 43, 32, 26],
                coeff,
                RGB8 { r: r as u8, g: g as u8, b: b as u8 },
                (duration as u64 * 1000u64).micros())
            );
        Ok(())
    }

    fn help(&self) -> &'static str {
        "<coeff> <R> <G> <B> <duration> - Let snake run with the given base color and coefficient of light off"
    }
}