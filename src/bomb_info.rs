use std::sync::OnceLock;

use crate::utils::read_string;

pub static SERIAL: OnceLock<Serial> = OnceLock::new();
pub static BATTERIES_COUNT: OnceLock<Batteries> = OnceLock::new();
pub static HAS_PARALLEL_PORT: OnceLock<Ports> = OnceLock::new();
pub static INDICATOR: OnceLock<Indicator> = OnceLock::new();

#[derive(Debug)]
pub struct Serial {
    // _inner: &'static str,
    contains_vowel: bool,
    last_num: u32,
}

impl Serial {
    fn new(s: &str) -> anyhow::Result<Self> {
        let last_char = s.chars().last().unwrap();
        if !last_char.is_ascii_digit() {
            anyhow::bail!("Serial last char is not digit.")
        }

        Ok(Self {
            // _inner: s,
            contains_vowel: s.chars().any(|c| "AEIOUaeiou".contains(c)),
            last_num: last_char.to_digit(10).unwrap(),
        })
    }
    pub fn is_contains_vowel(&self) -> bool {
        self.contains_vowel
    }

    /// True for even, false for odd.
    pub fn is_last_even(&self) -> bool {
        let last_num = self.last_num;
        last_num % 2 == 0
    }

    pub fn get_or_init() -> anyhow::Result<&'static Self> {
        Ok(match SERIAL.get() {
            Some(serial) => serial,
            None => {
                println!("Please input your serial number:");
                let s = read_string()?;
                let serial = Self::new(&s)?;
                SERIAL.set(serial).unwrap();
                SERIAL.get().unwrap()
            }
        })
    }
}

#[derive(Debug)]
pub struct Batteries {
    count: u32,
}

impl Batteries {
    fn new(s: &str) -> anyhow::Result<Self> {
        let count = s
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Batteries count must be a number."))?;
        Ok(Self { count })
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_or_init() -> anyhow::Result<&'static Self> {
        Ok(match BATTERIES_COUNT.get() {
            Some(batteries) => batteries,
            None => {
                println!("Please input your batteries count:");
                let s = read_string()?;
                let batteries = Self::new(&s)?;
                BATTERIES_COUNT.set(batteries).unwrap();
                BATTERIES_COUNT.get().unwrap()
            }
        })
    }
}

#[derive(Debug)]
pub struct Ports {
    has_parallel: bool,
}

impl Ports {
    pub fn new(s: &str) -> anyhow::Result<Self> {
        let has_parallel = match s {
            "y" => true,
            "n" => false,
            _ => anyhow::bail!("Invalid input, must be y/n.: {s}"),
        };
        Ok(Self { has_parallel })
    }

    pub fn has_parallel(&self) -> bool {
        self.has_parallel
    }

    pub fn get_or_init() -> anyhow::Result<&'static Self> {
        Ok(match HAS_PARALLEL_PORT.get() {
            Some(ports) => ports,
            None => {
                println!("Please input y/n has parallel port:");
                let s = read_string()?;
                let ports = Self::new(&s)?;
                HAS_PARALLEL_PORT.set(ports).unwrap();
                HAS_PARALLEL_PORT.get().unwrap()
            }
        })
    }
}

#[derive(Default, Debug)]
pub enum Indicator {
    #[default]
    Others,
    Car,
    Frk,
}

impl Indicator {
    pub fn new(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "CAR" => Indicator::Car,
            "FRK" => Indicator::Frk,
            _ => Indicator::Others,
        }
    }

    pub fn has_car(&self) -> bool {
        matches!(self, Indicator::Car)
    }

    pub fn has_frk(&self) -> bool {
        matches!(self, Indicator::Frk)
    }

    pub fn get_or_init() -> anyhow::Result<&'static Self> {
        Ok(match INDICATOR.get() {
            Some(indicator) => indicator,
            None => {
                println!("Please input your indicator:");
                let s = read_string()?;
                let indicator = Self::new(&s);
                INDICATOR.set(indicator).unwrap();
                INDICATOR.get().unwrap()
            }
        })
    }
}
