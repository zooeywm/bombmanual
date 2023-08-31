use std::sync::OnceLock;

pub static BOMBINFO: OnceLock<BombInfo> = OnceLock::new();

pub struct BombInfo {
    serial: Serial,
    /// Batteries now doesn't need to consider type.
    // batteries: Vec<Battery>,
    batteries_count: u32,
    has_parallel_port: bool,
}

struct Serial {
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
}

// #[derive(Debug)]
// enum Battery {
//     AA,
//     D,
// }

impl BombInfo {
    // /// Batteries now doesn't need to consider type.
    // fn new(serial: &str, batteries: &str) -> anyhow::Result<Self> {
    //     let batteries = batteries
    //         .split(',')
    //         .map(|b| match b {
    //             "AA" | "aa" => Ok(Battery::AA),
    //             "D" | "d" => Ok(Battery::D),
    //             _ => anyhow::bail!("Invalid battery type: {b}."),
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //     Ok(Self {
    //         serial: serial.to_string(),
    //         batteries,
    //     })
    // }

    fn new(serial: &str, batteries_count: &str, has_parallel_port: bool) -> anyhow::Result<Self> {
        let batteries_count = batteries_count
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Batteries count must be a number."))?;
        Ok(Self {
            serial: Serial::new(serial)?,
            batteries_count,
            has_parallel_port,
        })
    }

    pub fn init(s: &str, batteries_count: &str, has_parallel_port: bool) -> anyhow::Result<()> {
        let serial = Self::new(s, batteries_count, has_parallel_port)?;
        BOMBINFO
            .set(serial)
            .map_err(|_| anyhow::anyhow!("Serial has been initialized."))?;
        Ok(())
    }

    fn get() -> &'static BombInfo {
        BOMBINFO.get().unwrap()
        // .ok_or(anyhow::anyhow!("Serial not initialized."))
    }

    pub fn batteries_count() -> u32 {
        Self::get().batteries_count
    }

    pub fn is_serial_contains_vowel() -> bool {
        Self::get().serial.contains_vowel
    }

    /// True for even, false for odd.
    pub fn is_serial_last_even() -> bool {
        let last_num = Self::get().serial.last_num;
        last_num % 2 == 0
    }

    pub fn has_parallel_port() -> bool {
        Self::get().has_parallel_port
    }
}

#[cfg(test)]
mod tests {
    use crate::bomb_info::BombInfo;

    #[test]
    fn last_odd() {
        BombInfo::init("ABCDEF7", "0", false).unwrap();
        assert!(!BombInfo::is_serial_last_even());
    }

    #[test]
    fn last_even() {
        BombInfo::init("ABCDEF0", "0", false).unwrap();
        assert!(BombInfo::is_serial_last_even());
    }
}
