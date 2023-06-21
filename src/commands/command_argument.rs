use esp_println::{print, println};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CommandArgument<'d>
{
    pub data: &'d [char],
}

impl<'d> CommandArgument<'d>
{
    pub fn new(data: &'d [char]) -> Self
    {
        Self {
            data
        }
    }

    pub fn chars(&self) -> &'d [char]
    {
        self.data
    }

    pub fn try_to_integer(&self) -> Option<u32>
    {
        let length = self.data.len();
        let mut multiplier = 1u32;
        for _ in 0..length - 1 {
            multiplier *= 10;
        }

        let mut result = 0u32;

        for c in self.chars().iter() {
            let num = (*c as u32) - (b'0' as u32);
            if num > 9 {
                return None;
            }

            result += num * multiplier;
            multiplier /= 10;
        }

        Some(result)
    }

    pub fn compare(&self, to: &str) -> bool {
        if self.data.len() != to.len() {
            return false;
        }

        let to = to.as_bytes();
        for (i, c) in self.data.iter().enumerate() {
            let compare_against = to[i];

            if compare_against != (*c) as u8 {
                return false;
            }
        }

        true
    }
}