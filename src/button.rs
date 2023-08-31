use crate::bomb_info::{Batteries, Indicator};

pub struct Button {
    color: Color,
    text: Text,
}

#[derive(PartialEq)]
enum Color {
    Blue,
    White,
    Yellow,
    Red,
}

impl Color {
    fn new(s: &str) -> anyhow::Result<Self> {
        use Color::*;
        match s {
            "b" => Ok(Blue),
            "w" => Ok(White),
            "y" => Ok(Yellow),
            "r" => Ok(Red),
            _ => anyhow::bail!("Invalid input, must be a color： b/w/y/r."),
        }
    }
}

#[derive(PartialEq)]
enum Text {
    Abort,
    Detonate,
    Hold,
}

impl Text {
    fn new(s: &str) -> anyhow::Result<Self> {
        use Text::*;
        match s {
            "abort" | "中止" => Ok(Abort),
            "detonate" | "引爆" => Ok(Detonate),
            "hold" | "按住" => Ok(Hold),
            _ => anyhow::bail!(
                r#"Invalid input, must be one of the texts: "中止|abort"、"引爆|detonate"、"按住|hold"."#
            ),
        }
    }
}

enum Tactic {
    Touch,
    HoldUntil,
}

impl Button {
    pub fn new(s: &str) -> anyhow::Result<Self> {
        let mut iter = s.split_whitespace();
        let color = iter.next().ok_or(anyhow::anyhow!("No color text"))?;
        let text = iter.next().ok_or(anyhow::anyhow!("No label text"))?;
        Ok(Self {
            color: Color::new(color)?,
            text: Text::new(text)?,
        })
    }

    fn get_tactic(&self) -> anyhow::Result<Tactic> {
        use Color::*;
        use Tactic::*;
        use Text::*;
        Ok(if self.color == Blue && self.text == Abort {
            HoldUntil
        } else if Batteries::get_or_init()?.get_count() > 1 && self.text == Detonate {
            Touch
        } else if self.color == White && Indicator::get_or_init()?.has_car() {
            HoldUntil
        } else if Batteries::get_or_init()?.get_count() > 2 && Indicator::get_or_init()?.has_frk() {
            Touch
        } else if self.color == Yellow {
            HoldUntil
        } else if self.color == Red && self.text == Hold {
            Touch
        } else {
            HoldUntil
        })
    }

    pub fn get_countermeasure(&self) -> anyhow::Result<String> {
        let tactic = self.get_tactic()?;
        Ok(match tactic {
            Tactic::Touch => "Press and immediately release the button",
            Tactic::HoldUntil => {
                r#"Press and hold the button, watch the light bar on the right and refer to the following:
            Blue: Release when any digit of the timer shows 4;
            White: Release when any digit of the timer shows 1;
            Yellow: Release when any digit of the timer shows 5;
            Others: Release when any digit of the timer shows 1;
            "#
            }
        }
        .to_string())
    }
}
