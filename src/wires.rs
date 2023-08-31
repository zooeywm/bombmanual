use crate::bomb_info::Serial;

/// Line mode
pub struct Wires {
    inner: Vec<Color>,
}

/// Line color
#[derive(PartialEq)]
pub enum Color {
    Red,
    White,
    Blue,
    Dark,
    Yellow,
}

enum Command {
    One,
    MoreThanOne,
    No,
    Last,
}

impl Wires {
    pub fn new(s: &str) -> anyhow::Result<Self> {
        use Color::*;
        let colors = s
            .chars()
            .map(|c| match c {
                'r' => Ok(Red),
                'w' => Ok(White),
                'b' => Ok(Blue),
                'd' => Ok(Dark),
                'y' => Ok(Yellow),
                _ => anyhow::bail!("{s} has invalid color:{c}"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        if colors.len() > 6 || colors.len() < 3 {
            anyhow::bail!("Invalid length")
        }
        Ok(Self { inner: colors })
    }

    /// Get which wire should be cut.
    pub fn get_cut_num(&self) -> anyhow::Result<usize> {
        use Color::*;
        use Command::*;
        let wires = &self.inner;
        let len = wires.len();
        Ok(match len {
            3 => {
                // If there is no red wire, then cut the second wire.
                if self.condition(Red, No) {
                    1
                }
                // Otherwise, when the last line is white, cut the last line.
                else if self.condition(White, Last) {
                    len - 1
                }
                // Otherwise, when there is more than one blue wire, cut the last blue wire.
                else if self.condition(Blue, MoreThanOne) {
                    self.last_index(Blue)
                }
                // Otherwise, cut the last wire.
                else {
                    len - 1
                }
            }
            4 => {
                // If there is more than one red wire and the last digit of the serial number is odd, cut the last red wire
                if !self.condition(Red, MoreThanOne) && !Serial::get_or_init()?.is_last_even() {
                    self.last_index(Red)
                }
                //Otherwise, when there are no red wires and the last wire is yellow, cut the first wire.
                //Otherwise, when there is one and only one blue wire, cut the first wire.
                else if (self.condition(Red, No) && self.condition(Yellow, Last))
                    || self.condition(Blue, One)
                {
                    0
                }
                // Otherwise, when there is more than one yellow wire, cut the last one.
                else if !self.condition(Yellow, MoreThanOne) {
                    len - 1
                }
                //Otherwise, cut the second wire.
                else {
                    1
                }
            }
            5 => {
                // If the last wire is black and the last digit of the serial number is odd, cut the fourth wire
                if self.condition(Dark, Last) && !Serial::get_or_init()?.is_last_even() {
                    3
                }
                // Otherwise, when there is only one red wire and more than one yellow wire, cut the first wire.
                else if self.condition(Red, One) && self.condition(Yellow, MoreThanOne) {
                    0
                }
                // Otherwise, when there is no black wire, cut the second wire.
                else if self.condition(Dark, No) {
                    1
                }
                // Otherwise, cut the first wire.
                else {
                    0
                }
            }
            6 => {
                // If there is no yellow wire and the serial number ends in an odd number, cut the third wire.
                if self.condition(Yellow, No) && !Serial::get_or_init()?.is_last_even() {
                    2
                }
                // Otherwise, when there is only one yellow wire and more than one white wire, cut the fourth wire.
                else if self.condition(Yellow, One) && self.condition(White, MoreThanOne) {
                    3
                }
                // Otherwise, when there are no red wires, cut the last wire.
                else if self.condition(Red, No) {
                    len - 1
                }
                // Otherwise, cut the fourth wire.
                else {
                    3
                }
            }
            _ => unreachable!(),
        })
    }

    fn condition(&self, color: Color, command: Command) -> bool {
        let count = self.inner.iter().filter(|c| c.eq(&&color)).count();
        let last_color = self.inner.iter().last().unwrap();
        match command {
            Command::One => count == 1,
            Command::MoreThanOne => count > 1,
            Command::No => count == 0,
            Command::Last => last_color.eq(&color),
        }
    }

    fn last_index(&self, color: Color) -> usize {
        let mut index = 0;
        for (i, c) in self.inner.iter().enumerate() {
            if c.eq(&color) {
                index = i;
            }
        }
        index
    }
}
