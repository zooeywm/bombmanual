use crate::bomb_info::{Batteries, Ports, Serial};

enum Tactic {
    C,
    D,
    S,
    P,
    B,
}

pub struct ComplexWires {
    has_blue: bool,
    has_red: bool,
    had_star: bool,
    has_led: bool,
}

impl ComplexWires {
    pub fn new(s: &str) -> anyhow::Result<Self> {
        if s.chars().count() != 4 {
            anyhow::bail!("Invalid input, must be 4 characters of y/n.")
        };
        let bools = s
            .chars()
            .map(|el| match el {
                'y' => Ok(true),
                'n' => Ok(false),
                _ => anyhow::bail!("Invalid input, must be 4 characters of y/n."),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            has_blue: bools[0],
            has_red: bools[1],
            had_star: bools[2],
            has_led: bools[3],
        })
    }

    fn get_tactic(&self) -> Tactic {
        use Tactic::*;

        let Self {
            has_blue,
            has_red,
            had_star,
            has_led,
        } = self;

        match (
            *has_blue as u8,
            *has_red as u8,
            *had_star as u8,
            *has_led as u8,
        ) {
            (0, 0, 0, 0) => C,
            (0, 0, 0, 1) => D,
            (0, 0, 1, 0) => C,
            (0, 0, 1, 1) => B,
            (0, 1, 0, 0) => S,
            (0, 1, 0, 1) => B,
            (0, 1, 1, 0) => C,
            (0, 1, 1, 1) => B,
            (1, 0, 0, 0) => S,
            (1, 0, 0, 1) => P,
            (1, 0, 1, 0) => D,
            (1, 0, 1, 1) => P,
            (1, 1, 0, 0) => S,
            (1, 1, 0, 1) => S,
            (1, 1, 1, 0) => P,
            (1, 1, 1, 1) => D,
            _ => unreachable!(),
        }
    }

    pub fn is_cut(&self) -> anyhow::Result<bool> {
        use Tactic::*;
        let tactic = self.get_tactic();
        Ok(match tactic {
            C => true,
            D => false,
            S => Serial::get_or_init()?.is_last_even(),
            P => Ports::get_or_init()?.has_parallel(),
            B => Batteries::get_or_init()?.get_count() >= 2,
        })
    }
}
